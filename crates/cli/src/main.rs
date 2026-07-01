use anyhow::{Context, Result};
use chrono::SecondsFormat;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

use feedmind_domain::article::Article;
use feedmind_domain::feed::Feed;
use feedmind_domain::rules::{Rule, RuleAction};
use feedmind_domain::DecisionOutcome;
use feedmind_ingest::FeedFetcher;
use feedmind_opml::{OpmlDocument, OpmlExporter, OpmlOutline, OpmlParser};
use feedmind_rules::RuleEvaluator;

#[derive(Parser)]
#[command(name = "feedmind-cli")]
#[command(about = "FeedMind CLI - Import/Export and management tools")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Import feeds from an OPML file
    Import {
        /// Path to the OPML file
        #[arg(short, long)]
        file: PathBuf,

        /// User email to import feeds for (creates user if not exists)
        #[arg(short, long)]
        email: String,

        /// User password (only used if creating new user)
        #[arg(short, long, default_value = "changeme123")]
        password: String,
    },

    /// Export feeds to an OPML file
    Export {
        /// User email to export feeds for
        #[arg(short, long)]
        email: String,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Create a new user
    CreateUser {
        /// User email
        #[arg(short, long)]
        email: String,

        /// User password
        #[arg(short, long)]
        password: String,

        /// Subscription tier (free, pro_trial, pro, team)
        #[arg(short, long, default_value = "free")]
        tier: String,
    },

    /// Show database statistics
    Stats,

    /// Parse an OPML file and print a JSON summary without requiring a database
    OpmlSummary {
        /// Path to the OPML file
        #[arg(short, long)]
        file: PathBuf,
    },

    /// Fetch a feed and print a JSON summary without storing it
    FetchFeed {
        /// Feed URL to fetch
        #[arg(short, long)]
        url: String,
    },

    /// Evaluate one article JSON against one rule JSON without requiring a database
    EvaluateRule {
        /// Path to an Article JSON file
        #[arg(short, long)]
        article: PathBuf,

        /// Path to a Rule JSON file
        #[arg(short, long)]
        rule: PathBuf,
    },

    /// Build a deterministic CuratedItemExport demo without requiring a database or network
    DemoCurate {
        /// Path to the OPML subscriptions file used as source context
        #[arg(long)]
        opml: PathBuf,

        /// Path to an Article JSON file
        #[arg(long)]
        article: PathBuf,

        /// Path to a Rule JSON file
        #[arg(long)]
        rule: PathBuf,

        /// Output CuratedItemExport JSON path
        #[arg(short, long)]
        output: PathBuf,

        /// Actor reference to write in the export metadata
        #[arg(long, default_value = "actor-demo-local")]
        actor: String,
    },

    /// Fetch one live feed and export the first matching item; networked and not used by CI
    DemoCurateLive {
        /// RSS/Atom/JSON Feed URL to fetch
        #[arg(long)]
        feed_url: String,

        /// Path to a Rule JSON file
        #[arg(long)]
        rule: PathBuf,

        /// Output CuratedItemExport JSON path
        #[arg(short, long)]
        output: PathBuf,

        /// Actor reference to write in the export metadata
        #[arg(long, default_value = "actor-demo-live")]
        actor: String,

        /// Maximum fetched items to inspect before giving up
        #[arg(long, default_value_t = 20)]
        max_items: usize,
    },

    /// Validate the local CuratedItemExport contract invariants without requiring network access
    ValidateCuratedExport {
        /// Path to a CuratedItemExport JSON file
        #[arg(short, long)]
        file: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("feedmind_cli=info".parse()?))
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::OpmlSummary { file } => {
            opml_summary(&file)?;
        }
        Commands::FetchFeed { url } => {
            fetch_feed(&url).await?;
        }
        Commands::EvaluateRule { article, rule } => {
            evaluate_rule(&article, &rule)?;
        }
        Commands::DemoCurate {
            opml,
            article,
            rule,
            output,
            actor,
        } => {
            demo_curate(&opml, &article, &rule, &output, &actor)?;
        }
        Commands::DemoCurateLive {
            feed_url,
            rule,
            output,
            actor,
            max_items,
        } => {
            demo_curate_live(&feed_url, &rule, &output, &actor, max_items).await?;
        }
        Commands::ValidateCuratedExport { file } => {
            validate_curated_export(&file)?;
        }
        command => {
            let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await
                .context("Failed to connect to database")?;

            match command {
                Commands::Import {
                    file,
                    email,
                    password,
                } => {
                    import_opml(&pool, &file, &email, &password).await?;
                }
                Commands::Export { email, output } => {
                    export_opml(&pool, &email, &output).await?;
                }
                Commands::CreateUser {
                    email,
                    password,
                    tier,
                } => {
                    create_user(&pool, &email, &password, &tier).await?;
                }
                Commands::Stats => {
                    show_stats(&pool).await?;
                }
                Commands::OpmlSummary { .. }
                | Commands::FetchFeed { .. }
                | Commands::EvaluateRule { .. }
                | Commands::DemoCurate { .. }
                | Commands::DemoCurateLive { .. }
                | Commands::ValidateCuratedExport { .. } => unreachable!("handled before DB setup"),
            }
        }
    }

    Ok(())
}

#[derive(Serialize)]
struct OpmlSummary {
    title: Option<String>,
    feed_count: usize,
    folder_count: usize,
    feeds: Vec<OpmlSummaryFeed>,
}

#[derive(Serialize)]
struct OpmlSummaryFeed {
    title: String,
    xml_url: String,
    html_url: Option<String>,
    folder: Option<String>,
}

#[derive(Serialize)]
struct FetchFeedSummary {
    feed: FetchFeedMeta,
    item_count: usize,
    items: Vec<FetchFeedItem>,
}

#[derive(Serialize)]
struct FetchFeedMeta {
    url: String,
    title: String,
    feed_type: String,
    site_url: Option<String>,
}

#[derive(Serialize)]
struct FetchFeedItem {
    guid: String,
    title: String,
    url: Option<String>,
    published_at: Option<String>,
}

#[derive(Serialize)]
struct EvaluateRuleSummary {
    matched: bool,
    action: Option<RuleAction>,
    deciding_rule: Option<String>,
    decisions: Vec<EvaluateRuleDecision>,
}

#[derive(Serialize)]
struct EvaluateRuleDecision {
    rule_id: Uuid,
    outcome: String,
    actions: Vec<RuleAction>,
    confidence: f32,
    explanation: String,
    evidence: Vec<EvaluateRuleEvidence>,
}

#[derive(Serialize)]
struct EvaluateRuleEvidence {
    field: String,
    excerpt: String,
    pattern: Option<String>,
}

#[derive(Clone, Deserialize)]
struct RuleInput {
    user_id: Option<Uuid>,
    name: String,
    pattern: String,
    action: RuleAction,
    feed_id: Option<Uuid>,
    priority: Option<i32>,
    stop_on_match: Option<bool>,
}

#[derive(Serialize)]
struct CuratedItemExport {
    format: &'static str,
    export_id: String,
    origin_product: &'static str,
    workspace_id: String,
    created_by: String,
    created_at: String,
    purpose: &'static str,
    privacy_classification: &'static str,
    item: CuratedExportItem,
    source_ref: CuratedSourceRef,
    curation: CuratedExportCuration,
    rule_evidence: Vec<CuratedRuleEvidence>,
    constraints: CuratedExportConstraints,
    artifact_ref: CuratedArtifactRef,
    provenance_ref: CuratedProvenanceRef,
}

#[derive(Serialize)]
struct CuratedExportItem {
    item_id: String,
    title: String,
    content_excerpt: String,
    content_hash: String,
    source_url_hash: String,
    published_at: Option<String>,
    tags: Vec<String>,
}

#[derive(Serialize)]
struct CuratedSourceRef {
    source_id: String,
    source_type: &'static str,
    origin_product: &'static str,
    content_hash: String,
    provenance_id: String,
    opml_title: Option<String>,
    opml_feed_count: usize,
    first_feed_title: Option<String>,
}

#[derive(Serialize)]
struct CuratedExportCuration {
    decision: &'static str,
    reason: String,
    curated_by: String,
    curated_at: String,
}

#[derive(Serialize)]
struct CuratedRuleEvidence {
    rule_id: String,
    decision: &'static str,
    explanation: String,
    evidence_hash: String,
    confidence: f32,
}

#[derive(Serialize)]
struct CuratedExportConstraints {
    contains_raw_private_content: bool,
    contains_secrets: bool,
    contains_byok_material: bool,
    allow_downstream_execution: bool,
    data_residency: &'static str,
    retention_policy_ref: &'static str,
}

#[derive(Serialize)]
struct CuratedArtifactRef {
    artifact_id: String,
    artifact_type: &'static str,
    hash: String,
    manifest_ref: &'static str,
}

#[derive(Serialize)]
struct CuratedProvenanceRef {
    provenance_id: String,
    operation: &'static str,
    timestamp: String,
}

/// Flattened feed info for import
struct FlatFeed {
    title: String,
    xml_url: String,
    html_url: Option<String>,
    folder: Option<String>,
}

struct LiveSelection {
    feed: Feed,
    article: Article,
}

/// Flatten OPML outlines to a list of feeds with folder info
fn opml_summary(file: &PathBuf) -> Result<()> {
    let content = std::fs::read_to_string(file).context("Failed to read OPML file")?;
    let doc = OpmlParser::parse(&content).context("Failed to parse OPML file")?;
    let feeds = flatten_outlines(&doc.outlines, None);

    let summary = OpmlSummary {
        title: doc.title.clone(),
        feed_count: feeds.len(),
        folder_count: doc.folder_count(),
        feeds: feeds
            .into_iter()
            .map(|feed| OpmlSummaryFeed {
                title: feed.title,
                xml_url: feed.xml_url,
                html_url: feed.html_url,
                folder: feed.folder,
            })
            .collect(),
    };

    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

async fn fetch_feed(url: &str) -> Result<()> {
    let fetcher = FeedFetcher::new().context("Failed to create feed fetcher")?;
    let (feed, items) = fetcher
        .fetch(url)
        .await
        .with_context(|| format!("Failed to fetch feed: {url}"))?;

    let summary = FetchFeedSummary {
        feed: FetchFeedMeta {
            url: feed.url,
            title: feed.title,
            feed_type: feed.feed_type.to_string(),
            site_url: feed.site_url,
        },
        item_count: items.len(),
        items: items
            .into_iter()
            .take(20)
            .map(|item| FetchFeedItem {
                guid: item.guid,
                title: item.title,
                url: item.url,
                published_at: item.published_at.map(|date| date.to_rfc3339()),
            })
            .collect(),
    };

    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

fn evaluate_rule(article_file: &PathBuf, rule_file: &PathBuf) -> Result<()> {
    let article = load_article(article_file)?;
    let input = load_rule_input(rule_file)?;
    let rule = build_rule(&input);

    let evaluator = RuleEvaluator::new(vec![rule]).context("Failed to compile rule")?;
    let result = evaluator.evaluate(&article, article.feed_id);

    let summary = EvaluateRuleSummary {
        matched: result.action.is_some(),
        action: result.action,
        deciding_rule: result.deciding_rule,
        decisions: result
            .decisions
            .into_iter()
            .map(|decision| EvaluateRuleDecision {
                rule_id: decision.rule_id,
                outcome: format!("{:?}", decision.outcome),
                actions: decision.actions,
                confidence: decision.confidence,
                explanation: decision.explanation,
                evidence: decision
                    .evidence
                    .into_iter()
                    .map(|evidence| EvaluateRuleEvidence {
                        field: evidence.field,
                        excerpt: evidence.excerpt,
                        pattern: evidence.pattern,
                    })
                    .collect(),
            })
            .collect(),
    };

    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

fn demo_curate(
    opml_file: &PathBuf,
    article_file: &PathBuf,
    rule_file: &PathBuf,
    output: &PathBuf,
    actor: &str,
) -> Result<()> {
    let opml_content = std::fs::read_to_string(opml_file).context("Failed to read OPML file")?;
    let opml = OpmlParser::parse(&opml_content).context("Failed to parse OPML file")?;
    let feeds = flatten_outlines(&opml.outlines, None);
    anyhow::ensure!(
        !feeds.is_empty(),
        "OPML file must contain at least one feed for demo-curate"
    );

    let article = load_article(article_file)?;
    let input = load_rule_input(rule_file)?;
    let export = build_curated_export(&opml, &feeds, &article, &input, actor)?;
    write_curated_export(output, &export)?;

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "export_id": export.export_id,
            "output": output.display().to_string(),
            "matched": export.rule_evidence.iter().any(|evidence| evidence.decision == "match")
        }))?
    );
    Ok(())
}

async fn demo_curate_live(
    feed_url: &str,
    rule_file: &PathBuf,
    output: &PathBuf,
    actor: &str,
    max_items: usize,
) -> Result<()> {
    anyhow::ensure!(max_items > 0, "--max-items must be greater than zero");
    let input = load_rule_input(rule_file)?;
    let selection = select_live_article(feed_url, &input, max_items).await?;
    let opml = OpmlDocument {
        title: Some("FeedMind live demo subscriptions".to_string()),
        date_created: None,
        owner_email: None,
        outlines: vec![OpmlOutline::feed(
            selection.feed.title.clone(),
            selection.feed.url.clone(),
            selection.feed.site_url.clone(),
        )],
    };
    let feeds = vec![FlatFeed {
        title: selection.feed.title.clone(),
        xml_url: selection.feed.url.clone(),
        html_url: selection.feed.site_url.clone(),
        folder: None,
    }];
    let export = build_curated_export(&opml, &feeds, &selection.article, &input, actor)?;
    write_curated_export(output, &export)?;

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "export_id": export.export_id,
            "feed": selection.feed.title,
            "article": selection.article.title,
            "output": output.display().to_string(),
            "matched": export.rule_evidence.iter().any(|evidence| evidence.decision == "match")
        }))?
    );
    Ok(())
}

async fn select_live_article(
    feed_url: &str,
    input: &RuleInput,
    max_items: usize,
) -> Result<LiveSelection> {
    let fetcher = FeedFetcher::new().context("Failed to create feed fetcher")?;
    let (feed, items) = fetcher
        .fetch(feed_url)
        .await
        .with_context(|| format!("Failed to fetch feed: {feed_url}"))?;
    anyhow::ensure!(!items.is_empty(), "Fetched feed does not contain items");

    let evaluator =
        RuleEvaluator::new(vec![build_rule(input)]).context("Failed to compile rule")?;
    let fallback = items.first().cloned().expect("non-empty items checked");
    let selected = items
        .into_iter()
        .take(max_items)
        .map(|item| Article::from_feed_item(feed.id, item))
        .find(|article| {
            evaluator
                .evaluate(article, article.feed_id)
                .action
                .is_some()
        })
        .unwrap_or_else(|| Article::from_feed_item(feed.id, fallback));

    Ok(LiveSelection {
        feed,
        article: selected,
    })
}

fn write_curated_export(output: &PathBuf, export: &CuratedItemExport) -> Result<()> {
    let json = format!("{}\n", serde_json::to_string_pretty(export)?);

    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).context("Failed to create output directory")?;
        }
    }
    std::fs::write(output, json).context("Failed to write CuratedItemExport JSON")
}

fn validate_curated_export(file: &PathBuf) -> Result<()> {
    let content = std::fs::read_to_string(file).context("Failed to read CuratedItemExport JSON")?;
    let value: serde_json::Value = serde_json::from_str(&content).context("Invalid JSON")?;
    let object = value
        .as_object()
        .context("CuratedItemExport must be a JSON object")?;

    for field in [
        "format",
        "export_id",
        "origin_product",
        "created_by",
        "created_at",
        "purpose",
        "privacy_classification",
        "item",
        "source_ref",
        "curation",
        "rule_evidence",
        "constraints",
        "artifact_ref",
        "provenance_ref",
    ] {
        anyhow::ensure!(
            object.contains_key(field),
            "missing required field: {field}"
        );
    }
    anyhow::ensure!(
        object.get("format").and_then(|v| v.as_str()) == Some("feedmind.curated_item_export.v0.1"),
        "invalid CuratedItemExport format"
    );
    anyhow::ensure!(
        object.get("origin_product").and_then(|v| v.as_str()) == Some("rumble-feed-mind"),
        "invalid origin_product"
    );
    let constraints = object
        .get("constraints")
        .and_then(|v| v.as_object())
        .context("constraints must be an object")?;
    for field in [
        "contains_raw_private_content",
        "contains_secrets",
        "contains_byok_material",
        "allow_downstream_execution",
    ] {
        anyhow::ensure!(
            constraints.get(field).and_then(|v| v.as_bool()) == Some(false),
            "constraint must be false: {field}"
        );
    }
    let item = object
        .get("item")
        .and_then(|v| v.as_object())
        .context("item must be an object")?;
    for field in ["content_hash", "source_url_hash"] {
        let hash = item
            .get(field)
            .and_then(|v| v.as_str())
            .with_context(|| format!("item.{field} must be a string"))?;
        anyhow::ensure!(is_sha256_tag(hash), "item.{field} must be a sha256 tag");
    }
    let artifact_hash = object
        .get("artifact_ref")
        .and_then(|v| v.as_object())
        .and_then(|o| o.get("hash"))
        .and_then(|v| v.as_str())
        .context("artifact_ref.hash must be a string")?;
    anyhow::ensure!(
        is_sha256_tag(artifact_hash),
        "artifact_ref.hash must be a sha256 tag"
    );

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "file": file.display().to_string(),
            "valid": true,
            "format": "feedmind.curated_item_export.v0.1"
        }))?
    );
    Ok(())
}

fn is_sha256_tag(value: &str) -> bool {
    value
        .strip_prefix("sha256:")
        .is_some_and(|hex| hex.len() == 64 && hex.chars().all(|ch| ch.is_ascii_hexdigit()))
}

fn load_article(article_file: &PathBuf) -> Result<Article> {
    let article_content =
        std::fs::read_to_string(article_file).context("Failed to read article JSON")?;
    serde_json::from_str(&article_content).context("Invalid Article JSON")
}

fn load_rule_input(rule_file: &PathBuf) -> Result<RuleInput> {
    let rule_content = std::fs::read_to_string(rule_file).context("Failed to read rule JSON")?;
    serde_json::from_str(&rule_content).context("Invalid rule JSON")
}

fn build_rule(input: &RuleInput) -> Rule {
    let mut rule = Rule::new_regex(
        input.user_id.unwrap_or_else(Uuid::new_v4),
        input.name.clone(),
        input.pattern.clone(),
        input.action.clone(),
    );
    rule.feed_id = input.feed_id;
    rule.priority = input.priority.unwrap_or_default();
    rule.stop_on_match = input.stop_on_match.unwrap_or(false);
    rule
}

fn build_curated_export(
    opml: &OpmlDocument,
    feeds: &[FlatFeed],
    article: &Article,
    input: &RuleInput,
    actor: &str,
) -> Result<CuratedItemExport> {
    let evaluator =
        RuleEvaluator::new(vec![build_rule(input)]).context("Failed to compile rule")?;
    let result = evaluator.evaluate(article, article.feed_id);
    let first_decision = result.decisions.first();

    let content_material = format!(
        "{}\n{}\n{}",
        article.title,
        article.summary.as_deref().unwrap_or_default(),
        article.content.as_deref().unwrap_or_default()
    );
    let content_hash = sha256_tag(content_material.as_bytes());
    let source_url_hash = sha256_tag(article.url.as_deref().unwrap_or(&article.guid).as_bytes());
    let evidence_material = first_decision
        .map(|decision| {
            let evidence = decision
                .evidence
                .iter()
                .map(|item| {
                    format!(
                        "{}:{}:{}",
                        item.field,
                        item.excerpt,
                        item.pattern.as_deref().unwrap_or_default()
                    )
                })
                .collect::<Vec<_>>()
                .join("|");
            format!("{}:{}:{evidence}", input.name, decision.explanation)
        })
        .unwrap_or_else(|| format!("{}:{}:not_evaluated", input.name, input.pattern));
    let evidence_hash = sha256_tag(evidence_material.as_bytes());
    let export_id = format!("export:{}", article.guid);
    let provenance_id = format!("provenance:{export_id}");
    let artifact_hash = sha256_tag(
        format!("{export_id}:{content_hash}:{source_url_hash}:{evidence_hash}").as_bytes(),
    );
    let created_at = article
        .created_at
        .to_rfc3339_opts(SecondsFormat::Secs, true);
    let decision = first_decision
        .map(|decision| match decision.outcome {
            DecisionOutcome::Matched => "match",
            DecisionOutcome::NotMatched => "no_match",
            DecisionOutcome::Skipped => "not_evaluated",
        })
        .unwrap_or("not_evaluated");
    let confidence = first_decision
        .map(|decision| decision.confidence)
        .unwrap_or_default();
    let explanation = first_decision
        .map(|decision| decision.explanation.clone())
        .unwrap_or_else(|| "Rule was not evaluated".to_string());

    Ok(CuratedItemExport {
        format: "feedmind.curated_item_export.v0.1",
        export_id: export_id.clone(),
        origin_product: "rumble-feed-mind",
        workspace_id: "workspace:local-demo".to_string(),
        created_by: actor.to_string(),
        created_at: created_at.clone(),
        purpose: "local_export",
        privacy_classification: "normal",
        item: CuratedExportItem {
            item_id: article.id.to_string(),
            title: article.title.clone(),
            content_excerpt: content_excerpt(article),
            content_hash: content_hash.clone(),
            source_url_hash,
            published_at: article
                .published_at
                .map(|date| date.to_rfc3339_opts(SecondsFormat::Secs, true)),
            tags: article.categories.clone(),
        },
        source_ref: CuratedSourceRef {
            source_id: format!("source:{}", article.guid),
            source_type: "feed_item",
            origin_product: "rumble-feed-mind",
            content_hash,
            provenance_id: provenance_id.clone(),
            opml_title: opml.title.clone(),
            opml_feed_count: feeds.len(),
            first_feed_title: feeds.first().map(|feed| feed.title.clone()),
        },
        curation: CuratedExportCuration {
            decision: if result.action.is_some() {
                "saved"
            } else {
                "rejected"
            },
            reason: result
                .deciding_rule
                .unwrap_or_else(|| "No rule matched".to_string()),
            curated_by: actor.to_string(),
            curated_at: created_at.clone(),
        },
        rule_evidence: vec![CuratedRuleEvidence {
            rule_id: stable_rule_id(input),
            decision,
            explanation,
            evidence_hash,
            confidence,
        }],
        constraints: CuratedExportConstraints {
            contains_raw_private_content: false,
            contains_secrets: false,
            contains_byok_material: false,
            allow_downstream_execution: false,
            data_residency: "EU/local-first",
            retention_policy_ref: "retention:feedmind-local-demo",
        },
        artifact_ref: CuratedArtifactRef {
            artifact_id: format!("artifact:{export_id}"),
            artifact_type: "curated_export",
            hash: artifact_hash,
            manifest_ref: "manifest:feedmind-local-demo",
        },
        provenance_ref: CuratedProvenanceRef {
            provenance_id,
            operation: "exported",
            timestamp: created_at,
        },
    })
}

fn content_excerpt(article: &Article) -> String {
    let source = article
        .summary
        .as_deref()
        .or(article.content.as_deref())
        .unwrap_or(&article.title);
    source.chars().take(2_000).collect()
}

fn stable_rule_id(input: &RuleInput) -> String {
    format!(
        "rule:{}",
        &sha256_hex(format!("{}:{}", input.name, input.pattern).as_bytes())[..16]
    )
}

fn sha256_tag(bytes: &[u8]) -> String {
    format!("sha256:{}", sha256_hex(bytes))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn flatten_outlines(outlines: &[OpmlOutline], parent_folder: Option<&str>) -> Vec<FlatFeed> {
    let mut feeds = Vec::new();

    for outline in outlines {
        if outline.is_feed() {
            if let Some(xml_url) = &outline.xml_url {
                feeds.push(FlatFeed {
                    title: outline
                        .title
                        .clone()
                        .unwrap_or_else(|| outline.text.clone()),
                    xml_url: xml_url.clone(),
                    html_url: outline.html_url.clone(),
                    folder: parent_folder.map(|s| s.to_string()),
                });
            }
        }

        // Recurse into children (folders)
        if !outline.children.is_empty() {
            let folder_name = if outline.xml_url.is_none() {
                Some(outline.text.as_str())
            } else {
                parent_folder
            };
            feeds.extend(flatten_outlines(&outline.children, folder_name));
        }
    }

    feeds
}

async fn import_opml(
    pool: &sqlx::PgPool,
    file: &PathBuf,
    email: &str,
    password: &str,
) -> Result<()> {
    info!(opml_path = ?file, email_hash = %sha256_tag(email.as_bytes()), "Importing OPML");

    // Read OPML file
    let content = std::fs::read_to_string(file).context("Failed to read OPML file")?;

    // Parse OPML
    let doc = OpmlParser::parse(&content).context("Failed to parse OPML file")?;

    // Flatten the hierarchical structure
    let feeds = flatten_outlines(&doc.outlines, None);

    let folder_names: std::collections::HashSet<_> =
        feeds.iter().filter_map(|f| f.folder.as_ref()).collect();

    info!(
        "Parsed OPML: {} feeds in {} folders",
        feeds.len(),
        folder_names.len()
    );

    // Get or create user
    let user_id = get_or_create_user(pool, email, password).await?;
    info!("Using user ID: {}", user_id);

    // Create folders and feeds
    let mut folder_map: HashMap<String, Uuid> = HashMap::new();
    let mut feeds_created = 0;
    let mut feeds_skipped = 0;

    for feed in &feeds {
        // Create folder if needed
        let folder_id = if let Some(folder_name) = &feed.folder {
            if let Some(id) = folder_map.get(folder_name) {
                Some(*id)
            } else {
                let folder_id = create_folder(pool, user_id, folder_name).await?;
                folder_map.insert(folder_name.clone(), folder_id);
                Some(folder_id)
            }
        } else {
            None
        };

        // Create feed (skip if already exists)
        match create_feed(
            pool,
            user_id,
            folder_id,
            &feed.title,
            &feed.xml_url,
            feed.html_url.as_deref(),
        )
        .await
        {
            Ok(_) => feeds_created += 1,
            Err(e) => {
                if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
                    feeds_skipped += 1;
                } else {
                    tracing::warn!(feed_hash = %sha256_tag(feed.title.as_bytes()), error = %e, "Failed to create feed");
                    feeds_skipped += 1;
                }
            }
        }
    }

    info!(
        "Import complete: {} feeds created, {} skipped (duplicates), {} folders created",
        feeds_created,
        feeds_skipped,
        folder_map.len()
    );

    Ok(())
}

async fn export_opml(pool: &sqlx::PgPool, email: &str, output: &PathBuf) -> Result<()> {
    info!(email_hash = %sha256_tag(email.as_bytes()), output = ?output, "Exporting OPML");

    // Get user ID
    let user_id: Uuid =
        sqlx::query_scalar("SELECT id FROM users WHERE email = $1 AND deleted_at IS NULL")
            .bind(email)
            .fetch_one(pool)
            .await
            .context("User not found")?;

    // Get folders
    let folders: Vec<(Uuid, String)> =
        sqlx::query_as("SELECT id, name FROM folders WHERE user_id = $1 ORDER BY name")
            .bind(user_id)
            .fetch_all(pool)
            .await?;

    // Get feeds
    let feeds: Vec<(String, String, Option<String>, Option<Uuid>)> = sqlx::query_as(
        "SELECT title, url, site_url, folder_id FROM feeds WHERE user_id = $1 ORDER BY title",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    // Build folder map
    let folder_map: HashMap<Uuid, String> = folders.into_iter().collect();

    // Group feeds by folder
    let mut folder_feeds: HashMap<Option<Uuid>, Vec<OpmlOutline>> = HashMap::new();

    for (title, xml_url, html_url, folder_id) in feeds.iter() {
        let outline = OpmlOutline::feed(title.clone(), xml_url.clone(), html_url.clone());
        folder_feeds.entry(*folder_id).or_default().push(outline);
    }

    // Build outlines
    let mut outlines = Vec::new();

    // Root-level feeds (no folder)
    if let Some(root_feeds) = folder_feeds.remove(&None) {
        outlines.extend(root_feeds);
    }

    // Folder feeds
    for (folder_id, folder_name) in &folder_map {
        if let Some(feed_outlines) = folder_feeds.remove(&Some(*folder_id)) {
            let mut folder = OpmlOutline::folder(folder_name.clone());
            folder.children = feed_outlines;
            outlines.push(folder);
        }
    }

    let doc = OpmlDocument {
        title: Some(format!("FeedMind export for {}", email)),
        date_created: Some(chrono::Utc::now().to_rfc2822()),
        owner_email: Some(email.to_string()),
        outlines,
    };

    let xml = OpmlExporter::export(&doc);
    std::fs::write(output, xml)?;

    info!("Exported {} feeds to {:?}", feeds.len(), output);

    Ok(())
}

async fn create_user(pool: &sqlx::PgPool, email: &str, password: &str, tier: &str) -> Result<()> {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
        .to_string();

    let user_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO users (email, password_hash, tier)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
    )
    .bind(email)
    .bind(&password_hash)
    .bind(tier)
    .fetch_one(pool)
    .await
    .context("Failed to create user - email may already exist")?;

    info!(email_hash = %sha256_tag(email.as_bytes()), user_id = %user_id, "Created user");

    Ok(())
}

async fn show_stats(pool: &sqlx::PgPool) -> Result<()> {
    let users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE deleted_at IS NULL")
        .fetch_one(pool)
        .await?;

    let feeds: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM feeds")
        .fetch_one(pool)
        .await?;

    let articles: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM articles")
        .fetch_one(pool)
        .await?;

    let rules: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM rules")
        .fetch_one(pool)
        .await?;

    println!("FeedMind Statistics:");
    println!("  Users:    {}", users);
    println!("  Feeds:    {}", feeds);
    println!("  Articles: {}", articles);
    println!("  Rules:    {}", rules);

    Ok(())
}

async fn get_or_create_user(pool: &sqlx::PgPool, email: &str, password: &str) -> Result<Uuid> {
    // Try to get existing user
    let existing: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM users WHERE email = $1 AND deleted_at IS NULL")
            .bind(email)
            .fetch_optional(pool)
            .await?;

    if let Some(id) = existing {
        return Ok(id);
    }

    // Create new user
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
        .to_string();

    let user_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO users (email, password_hash, tier)
        VALUES ($1, $2, 'free')
        RETURNING id
        "#,
    )
    .bind(email)
    .bind(&password_hash)
    .fetch_one(pool)
    .await?;

    info!(email_hash = %sha256_tag(email.as_bytes()), user_id = %user_id, "Created new user");

    Ok(user_id)
}

async fn create_folder(pool: &sqlx::PgPool, user_id: Uuid, name: &str) -> Result<Uuid> {
    // Try to get existing folder
    let existing: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM folders WHERE user_id = $1 AND name = $2")
            .bind(user_id)
            .bind(name)
            .fetch_optional(pool)
            .await?;

    if let Some(id) = existing {
        return Ok(id);
    }

    // Create new folder
    let folder_id: Uuid =
        sqlx::query_scalar("INSERT INTO folders (user_id, name) VALUES ($1, $2) RETURNING id")
            .bind(user_id)
            .bind(name)
            .fetch_one(pool)
            .await?;

    Ok(folder_id)
}

async fn create_feed(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    folder_id: Option<Uuid>,
    title: &str,
    feed_url: &str,
    site_url: Option<&str>,
) -> Result<Uuid> {
    let feed_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO feeds (user_id, folder_id, title, url, site_url, feed_type)
        VALUES ($1, $2, $3, $4, $5, 'rss')
        RETURNING id
        "#,
    )
    .bind(user_id)
    .bind(folder_id)
    .bind(title)
    .bind(feed_url)
    .bind(site_url)
    .fetch_one(pool)
    .await?;

    Ok(feed_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_path(relative: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(relative)
    }

    #[test]
    fn demo_curate_builds_expected_curated_item_export() {
        let opml_content = std::fs::read_to_string(fixture_path("examples/demo.opml")).unwrap();
        let opml = OpmlParser::parse(&opml_content).unwrap();
        let feeds = flatten_outlines(&opml.outlines, None);
        let article = load_article(&fixture_path("examples/demo-article.json")).unwrap();
        let rule = load_rule_input(&fixture_path("examples/demo-rule.json")).unwrap();

        let export =
            build_curated_export(&opml, &feeds, &article, &rule, "actor-demo-local").unwrap();
        let actual = serde_json::to_value(export).unwrap();
        let expected: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(fixture_path("examples/expected-curated-export.json"))
                .unwrap(),
        )
        .unwrap();

        assert_eq!(actual, expected);
        assert_eq!(actual["constraints"]["contains_secrets"], false);
        assert_eq!(actual["constraints"]["contains_byok_material"], false);
        assert_eq!(actual["constraints"]["allow_downstream_execution"], false);
    }
}

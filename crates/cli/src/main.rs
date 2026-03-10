use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

use feedmind_domain::article::Article;
use feedmind_domain::rules::{Rule, RuleAction};
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
                | Commands::EvaluateRule { .. } => unreachable!("handled before DB setup"),
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

#[derive(Deserialize)]
struct RuleInput {
    user_id: Option<Uuid>,
    name: String,
    pattern: String,
    action: RuleAction,
    feed_id: Option<Uuid>,
    priority: Option<i32>,
    stop_on_match: Option<bool>,
}

/// Flattened feed info for import
struct FlatFeed {
    title: String,
    xml_url: String,
    html_url: Option<String>,
    folder: Option<String>,
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
    let article_content =
        std::fs::read_to_string(article_file).context("Failed to read article JSON")?;
    let rule_content = std::fs::read_to_string(rule_file).context("Failed to read rule JSON")?;

    let article: Article =
        serde_json::from_str(&article_content).context("Invalid Article JSON")?;
    let input: RuleInput = serde_json::from_str(&rule_content).context("Invalid rule JSON")?;

    let mut rule = Rule::new_regex(
        input.user_id.unwrap_or_else(Uuid::new_v4),
        input.name,
        input.pattern,
        input.action,
    );
    rule.feed_id = input.feed_id;
    rule.priority = input.priority.unwrap_or_default();
    rule.stop_on_match = input.stop_on_match.unwrap_or(false);

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
    info!("Importing OPML from {:?} for user {}", file, email);

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
                    tracing::warn!("Failed to create feed {}: {}", feed.title, e);
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
    info!("Exporting OPML for user {} to {:?}", email, output);

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

    info!("Created user {} with ID {}", email, user_id);

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

    info!("Created new user {} with ID {}", email, user_id);

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

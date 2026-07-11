use std::{env, str::FromStr};

use feedmind_storage::TenantTransaction;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{PgPool, Row};
use uuid::Uuid;

const TEST_DATABASE_URL: &str = "FEED_RADAR_TEST_DATABASE_URL";

async fn role_pool(
    database_url: &str,
    login: &str,
    role: &'static str,
    max_connections: u32,
) -> PgPool {
    let options = PgConnectOptions::from_str(database_url)
        .expect("test database URL must be valid")
        .username(login)
        .password("feed_radar_test_only");
    PgPoolOptions::new()
        .max_connections(max_connections)
        .after_connect(move |connection, _metadata| {
            Box::pin(async move {
                let statement = match role {
                    "feed_radar_owner" => "SET ROLE feed_radar_owner",
                    "feed_radar_app" => "SET ROLE feed_radar_app",
                    "feed_radar_auth" => "SET ROLE feed_radar_auth",
                    "feed_radar_worker" => "SET ROLE feed_radar_worker",
                    _ => return Err(sqlx::Error::Configuration("unknown test role".into())),
                };
                sqlx::query(statement).execute(connection).await?;
                Ok(())
            })
        })
        .connect_with(options)
        .await
        .expect("role pool must connect")
}

async fn provision_group_roles(admin: &PgPool) {
    for (name, statement) in [
        (
            "feed_radar_owner",
            "CREATE ROLE feed_radar_owner NOLOGIN NOINHERIT NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE",
        ),
        (
            "feed_radar_app",
            "CREATE ROLE feed_radar_app NOLOGIN NOINHERIT NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE",
        ),
        (
            "feed_radar_auth",
            "CREATE ROLE feed_radar_auth NOLOGIN NOINHERIT NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE",
        ),
        (
            "feed_radar_worker",
            "CREATE ROLE feed_radar_worker NOLOGIN NOINHERIT NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE",
        ),
        (
            "feed_radar_readonly",
            "CREATE ROLE feed_radar_readonly NOLOGIN NOINHERIT NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE",
        ),
    ] {
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_roles WHERE rolname = $1)")
            .bind(name)
            .fetch_one(admin)
            .await
            .expect("role lookup must succeed");
        if !exists {
            sqlx::query(statement)
                .execute(admin)
                .await
                .expect("test role provisioning must succeed");
        }
    }

    sqlx::query("GRANT USAGE, CREATE ON SCHEMA public TO feed_radar_owner")
        .execute(admin)
        .await
        .expect("migration owner must be able to create schema objects");
}

async fn provision_test_principals(admin: &PgPool) {
    for (name, create, grant) in [
        (
            "feed_radar_migrator_test",
            "CREATE ROLE feed_radar_migrator_test LOGIN NOINHERIT PASSWORD 'feed_radar_test_only'",
            "GRANT feed_radar_owner TO feed_radar_migrator_test",
        ),
        (
            "feed_radar_app_test",
            "CREATE ROLE feed_radar_app_test LOGIN NOINHERIT PASSWORD 'feed_radar_test_only'",
            "GRANT feed_radar_app TO feed_radar_app_test",
        ),
        (
            "feed_radar_auth_test",
            "CREATE ROLE feed_radar_auth_test LOGIN NOINHERIT PASSWORD 'feed_radar_test_only'",
            "GRANT feed_radar_auth TO feed_radar_auth_test",
        ),
        (
            "feed_radar_worker_test",
            "CREATE ROLE feed_radar_worker_test LOGIN NOINHERIT PASSWORD 'feed_radar_test_only'",
            "GRANT feed_radar_worker TO feed_radar_worker_test",
        ),
    ] {
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_roles WHERE rolname = $1)")
                .bind(name)
                .fetch_one(admin)
                .await
                .expect("test principal lookup must succeed");
        if !exists {
            sqlx::query(create)
                .execute(admin)
                .await
                .expect("test principal creation must succeed");
        }
        sqlx::query(grant)
            .execute(admin)
            .await
            .expect("test role membership must be granted");
    }
}

#[tokio::test]
async fn postgres_roles_rls_and_transaction_context_fail_closed() {
    let Ok(database_url) = env::var(TEST_DATABASE_URL) else {
        eprintln!("skipping PostgreSQL RLS test: {TEST_DATABASE_URL} is not set");
        return;
    };

    let admin = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .expect("admin test pool must connect");
    provision_group_roles(&admin).await;
    provision_test_principals(&admin).await;

    let unsafe_role_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM pg_roles
        WHERE rolname = ANY($1)
          AND (rolsuper OR rolbypassrls OR rolcanlogin OR rolinherit OR rolcreatedb OR rolcreaterole)
        "#,
    )
    .bind(&[
        "feed_radar_owner",
        "feed_radar_app",
        "feed_radar_auth",
        "feed_radar_worker",
        "feed_radar_readonly",
    ][..])
    .fetch_one(&admin)
    .await
    .expect("role attributes must be readable");
    assert_eq!(
        unsafe_role_count, 0,
        "runtime group roles must remain unprivileged"
    );

    let owner = role_pool(
        &database_url,
        "feed_radar_migrator_test",
        "feed_radar_owner",
        2,
    )
    .await;
    sqlx::migrate!("../../migrations")
        .run(&owner)
        .await
        .expect("migrations must run as feed_radar_owner");

    let rls_table_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM pg_class
        WHERE relname = ANY($1)
          AND relrowsecurity
          AND relforcerowsecurity
        "#,
    )
    .bind(
        &[
            "users",
            "folders",
            "feeds",
            "articles",
            "rules",
            "rule_evaluations",
            "tags",
            "article_tags",
            "sessions",
            "stripe_customers",
            "subscriptions",
            "usage_records",
            "usage_daily",
            "invoices",
            "payment_methods",
            "billing_events",
            "dunning_history",
            "feed_categories",
        ][..],
    )
    .fetch_one(&admin)
    .await
    .expect("RLS catalogue must be readable");
    assert_eq!(
        rls_table_count, 18,
        "every tenant table must enable and force RLS"
    );

    let tenant_a = Uuid::new_v4();
    let tenant_b = Uuid::new_v4();
    let feed_a = Uuid::new_v4();
    let feed_b = Uuid::new_v4();
    let article_a = Uuid::new_v4();
    let article_b = Uuid::new_v4();
    let tag_a = Uuid::new_v4();
    let tag_b = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO users (id, email, password_hash) VALUES ($1, $2, 'hash'), ($3, $4, 'hash')",
    )
    .bind(tenant_a)
    .bind(format!("tenant-a-{tenant_a}@example.invalid"))
    .bind(tenant_b)
    .bind(format!("tenant-b-{tenant_b}@example.invalid"))
    .execute(&owner)
    .await
    .expect("owner seed users must succeed");
    sqlx::query(
        "INSERT INTO feeds (id, user_id, url, title) VALUES ($1, $2, $3, 'A'), ($4, $5, $6, 'B')",
    )
    .bind(feed_a)
    .bind(tenant_a)
    .bind(format!("https://a-{feed_a}.invalid/feed"))
    .bind(feed_b)
    .bind(tenant_b)
    .bind(format!("https://b-{feed_b}.invalid/feed"))
    .execute(&owner)
    .await
    .expect("owner seed feeds must succeed");
    sqlx::query(
        "INSERT INTO articles (id, feed_id, user_id, guid, title) VALUES ($1, $2, $3, $4, 'A'), ($5, $6, $7, $8, 'B')",
    )
    .bind(article_a)
    .bind(feed_a)
    .bind(tenant_a)
    .bind(format!("guid-{article_a}"))
    .bind(article_b)
    .bind(feed_b)
    .bind(tenant_b)
    .bind(format!("guid-{article_b}"))
    .execute(&owner)
    .await
    .expect("owner seed articles must succeed");
    sqlx::query("INSERT INTO tags (id, user_id, name) VALUES ($1, $2, $3), ($4, $5, $6)")
        .bind(tag_a)
        .bind(tenant_a)
        .bind(format!("tag-{tag_a}"))
        .bind(tag_b)
        .bind(tenant_b)
        .bind(format!("tag-{tag_b}"))
        .execute(&owner)
        .await
        .expect("owner seed tags must succeed");

    let app = role_pool(&database_url, "feed_radar_app_test", "feed_radar_app", 1).await;
    let (session_role, current_role): (String, String) =
        sqlx::query_as("SELECT session_user::text, current_user::text")
            .fetch_one(&app)
            .await
            .expect("runtime identity must be readable");
    assert_eq!(session_role, "feed_radar_app_test");
    assert_eq!(current_role, "feed_radar_app");

    let visible_without_context: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM feeds")
        .fetch_one(&app)
        .await
        .expect("missing context must return no rows, not bypass RLS");
    assert_eq!(visible_without_context, 0);

    let mut tx_a = TenantTransaction::begin(&app, tenant_a)
        .await
        .expect("tenant A transaction must begin");
    let visible_a: Vec<Uuid> = sqlx::query_scalar("SELECT user_id FROM feeds ORDER BY user_id")
        .fetch_all(tx_a.connection())
        .await
        .expect("tenant A query must succeed");
    assert_eq!(visible_a, vec![tenant_a]);

    let changed_b = sqlx::query("UPDATE feeds SET title = 'compromised' WHERE id = $1")
        .bind(feed_b)
        .execute(tx_a.connection())
        .await
        .expect("cross-tenant update must be filtered by RLS");
    assert_eq!(changed_b.rows_affected(), 0);

    let cross_feed_article = sqlx::query(
        "INSERT INTO articles (feed_id, user_id, guid, title) VALUES ($1, $2, $3, 'cross')",
    )
    .bind(feed_b)
    .bind(tenant_a)
    .bind(format!("cross-{tenant_a}"))
    .execute(tx_a.connection())
    .await;
    assert!(
        cross_feed_article.is_err(),
        "composite tenant foreign keys must reject a tenant B feed"
    );
    tx_a.rollback()
        .await
        .expect("failed transaction must roll back");

    let mut derived_tx = TenantTransaction::begin(&app, tenant_a)
        .await
        .expect("derived-policy transaction must begin");
    let cross_tag = sqlx::query("INSERT INTO article_tags (article_id, tag_id) VALUES ($1, $2)")
        .bind(article_a)
        .bind(tag_b)
        .execute(derived_tx.connection())
        .await;
    assert!(
        cross_tag.is_err(),
        "derived ownership must reject a tenant B tag"
    );
    derived_tx
        .rollback()
        .await
        .expect("failed transaction must roll back");

    let mut committed = TenantTransaction::begin(&app, tenant_a)
        .await
        .expect("tenant transaction must begin");
    let _: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM articles")
        .fetch_one(committed.connection())
        .await
        .expect("tenant query must succeed");
    committed
        .commit()
        .await
        .expect("tenant transaction must commit");

    let leaked: Option<String> = sqlx::query_scalar("SELECT current_setting('app.user_id', true)")
        .fetch_one(&app)
        .await
        .expect("tenant setting lookup must succeed");
    assert!(
        leaked.as_deref().unwrap_or_default().is_empty(),
        "tenant context leaked through pool"
    );

    let mut malformed = app.begin().await.expect("transaction must begin");
    sqlx::query("SELECT set_config('app.user_id', 'not-a-uuid', true)")
        .execute(&mut *malformed)
        .await
        .expect("PostgreSQL accepts the string before policy evaluation");
    let malformed_result = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM feeds")
        .fetch_one(&mut *malformed)
        .await;
    assert!(
        malformed_result.is_err(),
        "malformed tenant context must fail closed"
    );
    malformed
        .rollback()
        .await
        .expect("malformed transaction must roll back");

    let auth = role_pool(&database_url, "feed_radar_auth_test", "feed_radar_auth", 1).await;
    let direct_auth_read = sqlx::query("SELECT id FROM users LIMIT 1")
        .fetch_optional(&auth)
        .await;
    assert!(
        direct_auth_read.is_err(),
        "auth role must not read users directly"
    );
    let found_tenant: Uuid = sqlx::query("SELECT id FROM feed_radar_auth_find_user($1)")
        .bind(format!("tenant-b-{tenant_b}@example.invalid"))
        .fetch_one(&auth)
        .await
        .expect("reviewed auth function must be executable")
        .get("id");
    assert_eq!(found_tenant, tenant_b);

    let worker = role_pool(
        &database_url,
        "feed_radar_worker_test",
        "feed_radar_worker",
        1,
    )
    .await;
    let worker_feed_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM feeds")
        .fetch_one(&worker)
        .await
        .expect("worker's explicit cross-tenant feed read must succeed");
    assert!(worker_feed_count >= 2);
    let worker_folder_read = sqlx::query("SELECT id FROM folders LIMIT 1")
        .fetch_optional(&worker)
        .await;
    assert!(
        worker_folder_read.is_err(),
        "worker must not read tables outside its enumerated grants"
    );
    let worker_schema_change = sqlx::query("ALTER TABLE feeds ADD COLUMN forbidden BOOLEAN")
        .execute(&worker)
        .await;
    assert!(
        worker_schema_change.is_err(),
        "worker must not alter schema"
    );
}

use sqlx::PgPool;

pub async fn fetch_all_markets(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT market FROM markets ORDER BY market")
        .fetch_all(pool)
        .await
}

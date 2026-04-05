use anyhow::Result;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect("sqlite:cuisine.db").await?;
    sqlx::migrate!().run(&pool).await?;

    let app = cuisine::create_app(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}

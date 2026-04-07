use anyhow::Result;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect("sqlite:cuisine.db").await?;
    sqlx::migrate!().run(&pool).await?;

    let app = cuisine::create_app(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.map_err(|e| {
        eprintln!("Error: failed to bind port 3000 — {e}");
        eprintln!("Hint: another process may already be using port 3000.");
        eprintln!("      Run: lsof -i :3000  or  ss -tlnp | grep 3000");
        e
    })?;

    tracing::info!("listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}

mod handlers;

use anyhow::Result;
use axum::routing::get;
use axum::Extension;
use axum::Router;
use handlers::blog::get_blog_post_handler;
use handlers::blog::get_blog_posts_handler;

async fn get_connection_pool(url: &str) -> Result<sqlx::SqlitePool> {
    let connection_pool = sqlx::SqlitePool::connect(url).await?;

    Ok(connection_pool)
}

async fn run_migrations(pool: sqlx::SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}

async fn create_router(pool: sqlx::SqlitePool) -> Result<Router> {
    let app = axum::Router::new()
        .route("/blogs", get(get_blog_posts_handler))
        .route("/blogs/:id", get(get_blog_post_handler))
        .layer(Extension(pool.clone()));

    Ok(app)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let listen_address = std::env::var("LISTEN_ADDRESS")?;
    let listener = tokio::net::TcpListener::bind(&listen_address).await?;
    let pool = get_connection_pool(&database_url).await?;

    run_migrations(pool.clone()).await?;

    println!("Listening on: {}", listen_address);

    let router = create_router(pool.clone()).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

use axum::{
    routing::{get, post},
    Router,
};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod entity;
mod migration;
mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: DatabaseConnection = Database::connect(&db_url).await?;

    // Run migrations here in the future
    use sea_orm_migration::MigratorTrait;
    migration::Migrator::up(&db, None).await?;

    let state = AppState { db };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/todos", get(handlers::todo::list_todos).post(handlers::todo::create_todo))
        .route("/todos/:id", get(handlers::todo::get_todo).put(handlers::todo::update_todo).delete(handlers::todo::delete_todo))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

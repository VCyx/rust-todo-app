use axum::{extract::FromRef, http::StatusCode, routing::get, Router};
use leptos::provide_context;
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos_config::{Env, LeptosOptions};
use sea_orm::{Database, DatabaseConnection};
use std::{env, net::SocketAddr, path::PathBuf};
use tower_http::services::ServeDir;
use ui::app::App;

mod entity;
mod migration;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub leptos_options: LeptosOptions,
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await?;

    use sea_orm_migration::MigratorTrait;
    migration::Migrator::up(&db, None).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let leptos_options = leptos_options(addr);
    let static_files_dir = client_dist_dir();
    let state = AppState {
        db: db.clone(),
        leptos_options: leptos_options.clone(),
    };
    let routes = generate_route_list(App);
    let db_context = db.clone();

    let app = Router::new()
        .route("/favicon.ico", get(no_favicon))
        .leptos_routes_with_context(
            &state,
            routes,
            move || provide_context(db_context.clone()),
            App,
        )
        .fallback_service(ServeDir::new(static_files_dir))
        .with_state(state);

    tracing::info!("Server listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn no_favicon() -> StatusCode {
    StatusCode::NO_CONTENT
}

fn leptos_options(addr: SocketAddr) -> LeptosOptions {
    LeptosOptions::builder()
        .output_name("ui")
        .site_root(client_dist_dir().to_string_lossy().to_string())
        .site_pkg_dir(".")
        .env(if cfg!(debug_assertions) { Env::DEV } else { Env::PROD })
        .site_addr(addr)
        .hash_files(false)
        .build()
}

fn client_dist_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("ui")
        .join("dist")
}

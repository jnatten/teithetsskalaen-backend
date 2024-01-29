use crate::config::Config;
use crate::repository::Repository;
use crate::service::write_service::WriteService;
use crate::Error;
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let repository = Arc::new(Repository::new(db));
    let write_service = Arc::new(WriteService::new(repository.clone()));

    let state = Arc::new(AppState {
        repository,
        write_service,
    });

    let app = Router::new()
        .nest("/teithet", crate::controller::teithet::router())
        .with_state(state);

    let address = format!("0.0.0.0:{}", config.port);
    println!("Serving application on {}", address);
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

pub struct AppState {
    pub repository: Arc<Repository>,
    pub write_service: Arc<WriteService>,
}

pub type ApiResult<T, E = Error> = ::std::result::Result<T, E>;

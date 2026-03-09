use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Router,
};
use rudi::{Context, Singleton};
use tokio::{net::TcpListener, sync::Mutex};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

trait Service: Send + Sync {
    fn insert(&self, name: String) -> impl std::future::Future<Output = ()> + Send;
    fn search(&self, name: &str) -> impl std::future::Future<Output = Option<String>> + Send;
    fn delete(&self, name: &str) -> impl std::future::Future<Output = ()> + Send;
}

#[derive(Clone)]
#[Singleton]
struct ServiceImpl {
    db: Arc<Mutex<Vec<String>>>,
}

impl Service for ServiceImpl {
    async fn insert(&self, name: String) {
        self.db.lock().await.push(name);
    }

    async fn search(&self, name: &str) -> Option<String> {
        self.db
            .lock()
            .await
            .iter()
            .find(|n| n.contains(name))
            .cloned()
    }

    async fn delete(&self, name: &str) {
        self.db.lock().await.retain(|n| n != name);
    }
}

async fn insert(Path(name): Path<String>, State(svc): State<ServiceImpl>) {
    svc.insert(name).await;
}

async fn search(Path(name): Path<String>, State(svc): State<ServiceImpl>) -> String {
    svc.search(&name).await.unwrap_or("".to_string())
}

async fn del(Path(name): Path<String>, State(svc): State<ServiceImpl>) {
    svc.delete(&name).await;
}

#[Singleton]
fn EmptyVec() -> Arc<Mutex<Vec<String>>> {
    Arc::new(Mutex::new(Vec::new()))
}

#[Singleton]
async fn Run(svc: ServiceImpl) {
    let app = Router::new()
        .route("/insert/{name}", post(insert))
        .route("/search/{name}", get(search))
        .route("/delete/{name}", delete(del))
        .with_state(svc);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_example=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut cx = Context::auto_register();

    // cx.resolve_async::<()>().await;
    cx.resolve_async().await
}

use axum::Router;
use axum::routing::{get,post};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

use crate::router::{
    router_to_link,
    create_router,
};
use encurta::db_connection;
use crate::web::app;
use dotenvy::dotenv;

mod web;
mod router;

#[tokio::main]
async fn main() {
      dotenv().ok();
      let info_file = rolling::daily("./logs", "info").with_max_level(tracing::Level::INFO);
      
      tracing_subscriber::fmt()
        .with_target(false)
        .with_writer(info_file)
        .with_ansi(false)
        .compact()
        .init();

    let api_v1_routes = Router::new()
         .route("/router/{hash}", get(router_to_link))
         .route("/router", post(create_router));
      
    let app = Router::new()
           .route("/", get(app))
           .nest("/api/v1", api_v1_routes)
           .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new()
                    .level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new()
                    .level(Level::INFO)),

            );
  
    let conn = db_connection().await;
    let _ = conn
        .execute(
            "CREATE TABLE IF NOT EXISTS urls(link varchar non null, hash varchar non null)",
            (),
        )
        .await;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


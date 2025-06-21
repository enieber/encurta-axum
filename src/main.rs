use sha256::{digest};
use axum::{
    http::StatusCode,
    extract,
    extract::Path,
    routing::{get,post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use libsql::{params, Connection, Database};
use std::env;
use dotenvy::dotenv;
use axum::response::IntoResponse;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
  
#[tokio::main]
async fn main() {
      dotenv().ok();
      tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let api_v1_routes = Router::new()
         .route("/router/{hash}", get(router_to_link))
         .route("/router", post(create_router));
      
    let app = Router::new()
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


#[derive(Deserialize, Serialize)]
struct NewRouter {
    link: String,
}


#[derive(Deserialize, Serialize)]
struct GenerateRouter {
    link: String,
    hash: String,
}


fn generate_hash_link(link: String) -> String {
    let hash = digest(link.to_string()).to_string();
    let new_hash = &hash[0..7];
    return new_hash.to_string()
}


async fn create_router(extract::Json(payload): extract::Json<NewRouter>) -> impl IntoResponse {  
   let hash = generate_hash_link(payload.link.to_string());
   let conn = db_connection().await;
   let link_option = get_link_from_hash(conn.clone(), hash.clone()).await;
   match link_option {
       Some(link) => {
         let router = GenerateRouter {
           hash,
           link: link.to_string()
         };

         return (StatusCode::OK, Json(router))
       },
       None => {
         let router = GenerateRouter {
           hash: hash.clone(),
           link: payload.link.to_string()
         };

        let _ = conn
        .query("INSERT into urls values (?1, ?2)", params![payload.link.clone(), hash])
        .await;

         return (StatusCode::OK, Json(router))
     }
   }
}

async fn get_link_from_hash(conn: Connection, hash: String) -> Option<String> {
    let query = format!("SELECT * FROM urls WHERE hash = '{}'", hash);
    
    let mut results = match conn.query(query.as_str(), ()).await {
        Ok(results) => results,
        Err(_) => return None,
    };
    
    let row = match results.next().await {
        Ok(Some(row)) => row,
        _ => return None,
    };
    
    match row.get(0) {
        Ok(link) => Some(link),
        Err(_) => None,
    }
}

async fn router_to_link(Path(hash): Path<String>) -> impl IntoResponse {
   let conn = db_connection().await;
   let link_option = get_link_from_hash(conn, hash.clone()).await;
   match link_option {
       Some(link) => {
         let router = GenerateRouter {
           hash,
           link: link.to_string()
         };

         return (StatusCode::OK, Json(router))
       },
       None => {
         let router = GenerateRouter {
            hash: hash.clone(),
            link: String::from("")
         };
         return (StatusCode::NOT_FOUND, Json(router))
     }
   }
}

async fn db_connection() -> Connection {
    dotenv().expect(".env file not found");

    let db_url = env::var("DATABASE_URL").unwrap();

    let db = Database::open(db_url).unwrap();

    db.connect().unwrap()
}

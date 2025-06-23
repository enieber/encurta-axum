use sha256::{digest};
use axum::{
    http::StatusCode,
    extract,
    extract::Path,
    Json,
};
use serde::{Deserialize, Serialize};
use libsql::{params, Connection};
use axum::response::IntoResponse;
use encurta::db_connection;

#[derive(Deserialize, Serialize)]
pub struct NewRouter {
    pub link: String,
}


#[derive(Deserialize, Serialize)]
pub struct GenerateRouter {
    link: String,
    hash: String,
}


fn generate_hash_link(link: String) -> String {
    let hash = digest(link.to_string()).to_string();
    let new_hash = &hash[0..7];
    return new_hash.to_string()
}


pub async fn create_router(extract::Json(payload): extract::Json<NewRouter>) -> impl IntoResponse {  
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

        let link_clone = payload.link.clone();
        let hash_clone = hash.clone();
        let _ = conn
        .query("INSERT into urls values (?1, ?2)", params![link_clone, hash_clone])
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

pub async fn router_to_link(Path(hash): Path<String>) -> impl IntoResponse {
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


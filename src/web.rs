use dioxus::prelude::*;
use axum::response::Html;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use crate::router::NewRouter;
use tracing::{info};

fn send_request(url: String) {
      
    spawn(async move {
                let router = NewRouter {
                    link: url.to_string(),
                };
                let mut headers = HeaderMap::new();
                headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
                let resp = reqwest::Client::new()
                    .post("/api/v1/router")
                    .headers(headers)
                    .json(&router)
                    .send()
                    .await;

                if resp.is_ok() {
                    info!("dioxuslabs.com responded!");
                } else  {
                    info!("failed to fetch response!");
                }
              });
}

#[component]
fn InputUrl() -> Element {
    let mut url = use_signal(|| "".to_string());

    let on_submit = move |event: FormEvent| {
        event.prevent_default();
        send_request(url.to_string());
        info!("Submitted! {event:?}");
    };
   
    rsx! {
        form { onsubmit: on_submit,
            input {
                value: "{url}",
                oninput: move |event| url.set(event.value())
            }
             button { type: "submit", "Encutrar" }
        }
    }
}

pub async fn app() -> Html<String> {
    // render the rsx! macro to HTML
       
    Html(dioxus_ssr::render_element(
        rsx! {
            h1 { "Encurta url"}
            InputUrl {}
        }
    ))
}

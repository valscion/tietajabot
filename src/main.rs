use chrono::NaiveDateTime;
use lambda_http::{lambda, IntoResponse, Request, RequestExt};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;
use telegram_typings;

#[cfg_attr(tarpaulin, skip)]
fn main() {
    lambda!(handler)
}

use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
struct ApiGatewayBody {
    #[serde(default = "default_api_gateway_body")]
    body: String,
}
fn default_api_gateway_body() -> String {
    "{}".to_string()
}

#[derive(Deserialize, Default, Debug)]
struct TelegramUpdates {
    #[serde(default)]
    ok: bool,
    #[serde(default)]
    result: Vec<telegram_typings::Update>,
}

fn handler(request: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    let body: ApiGatewayBody = request
        .payload()
        .unwrap_or_else(|_parse_err| None)
        .unwrap_or_default();

    let update: TelegramUpdates = serde_json::from_str(body.body.as_str())?;
    if update.ok != true {
        let raw_payload: String = request.payload().unwrap().expect("No payload received");
        let error: String = json!({
            "ok": false,
            "request_payload": raw_payload
        })
        .to_string();
        return Err(HandlerError::from(error.as_str()));
    }

    for update in &update.result {
        match &update.message {
            Some(msg) => {
                let from = &msg.from;
                let chat = format!("({}, {})", &msg.chat.id, &msg.chat.type_tl);
                let text = &msg.text;
                let date = NaiveDateTime::from_timestamp(msg.date, 0);
                print!("[{}] {:?} in chat {} sent: {:?}", date, from, chat, text);
            }
            None => print!("Not a message, ignoring..."),
        }
    }
    Ok(json!({"ok": true}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http::header::CONTENT_TYPE;

    #[test]
    fn handler_handles() {
        let body = lambda_http::Body::from(include_str!("../fixtures/request.json"));
        let mut request = Request::new(body);
        request
            .headers_mut()
            .insert(CONTENT_TYPE, "application/json".parse().unwrap());
        let response = handler(request, Context::default())
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), json!({"ok": true}).into_response().body())
    }
}

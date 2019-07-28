#[macro_use]
extern crate log;
extern crate env_logger;

use chrono::NaiveDateTime;
use lambda_http::{lambda, IntoResponse, Request, RequestExt};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;
use telegram_typings;

#[cfg_attr(tarpaulin, skip)]
fn main() {
    env_logger::init();
    lambda!(handler)
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default, Debug)]
struct ApiGatewayBody {
    #[serde(default = "default_api_gateway_body")]
    body: String,
}
fn default_api_gateway_body() -> String {
    "{}".to_string()
}

#[derive(Serialize, Deserialize)]
struct TelegramSuccessResponse {
    /// Should always be `true` or Telegram gave a strange response
    ok: bool,
    /// Contains the list of received updates
    result: Vec<telegram_typings::Update>,
}

#[derive(Serialize, Deserialize)]
struct TelegramErrorResponse {
    /// Should always be `false` or Telegram gave a strange response
    ok: bool,
    /// The human-readable description of the erroneus result
    description: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum TelegramResponse {
    Success(TelegramSuccessResponse),
    Error(TelegramErrorResponse),
}

fn handler(request: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    let body: ApiGatewayBody = request
        .payload()
        .unwrap_or_else(|_parse_err| None)
        .unwrap_or_default();

    let body_value: serde_json::Value =
        serde_json::from_str(body.body.as_str()).expect("Body was not a valid JSON value");
    debug!(
        "Received JSON: {}",
        serde_json::to_string(&body_value).unwrap()
    );

    let possibly_updates: Result<TelegramResponse, serde_json::Error> =
        serde_json::from_value(body_value);
    let updates = match possibly_updates {
        Err(json_err) => {
            error!("JSON conversion error: {:#?}", json_err);
            return Ok(json!({"ok": false}));
        }
        Ok(updates) => match updates {
            TelegramResponse::Error(error) => {
                error!("TELEGRAM_ERROR: {}", error.description);
                return Ok(json!({
                    "ok": false,
                    "description": error.description
                }));
            }
            TelegramResponse::Success(success) => success,
        },
    };

    for update in &updates.result {
        if let Some(msg) = &update.message {
            if let Some(from) = &msg.from {
                if let Some(text) = &msg.text {
                    let from = format_name_for_user(from);
                    let time = NaiveDateTime::from_timestamp(msg.date, 0);
                    info!(
                        "Received a message sent at {time} from {from}: {text}",
                        time = time,
                        from = from,
                        text = text,
                    );
                }
            }
        }
    }
    Ok(json!({"ok": true}))
}

fn format_name_for_user(user: &telegram_typings::User) -> String {
    if let Some(username) = &user.username {
        if let Some(last_name) = &user.last_name {
            format!(
                "@{username} ({first} {last})",
                username = username,
                first = user.first_name,
                last = last_name
            )
        } else {
            format!(
                "@{username} ({first})",
                username = username,
                first = user.first_name
            )
        }
    } else {
        if let Some(last_name) = &user.last_name {
            format!("{first} {last}", first = user.first_name, last = last_name)
        } else {
            format!("{first}", first = user.first_name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::http::header::CONTENT_TYPE;

    fn setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn handler_handles() {
        setup();

        let request_json = include_str!("../fixtures/request.json");
        let mut request_value: serde_json::Value =
            serde_json::from_str(&request_json).expect("request.json parse failed");
        let update_json = include_str!("../fixtures/updates.json");
        let _update_value: serde_json::Value =
            serde_json::from_str(&update_json).expect("updates.json was not valid JSON");

        *request_value.get_mut("body").expect("get_mut(body) failed") =
            serde_json::to_value(update_json).unwrap();
        let request_string = serde_json::to_string(&request_value)
            .expect("Failed to serialize request_value to string");

        let body = lambda_http::Body::from(request_string);

        let mut request = Request::new(body);
        request
            .headers_mut()
            .insert(CONTENT_TYPE, "application/json".parse().unwrap());
        let response = handler(request, Context::default())
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), json!({"ok": true}).into_response().body())
    }

    #[test]
    fn handler_handles_errors() {
        setup();

        let request_json = include_str!("../fixtures/request.json");
        let mut request_value: serde_json::Value =
            serde_json::from_str(&request_json).expect("request.json parse failed");

        *request_value.get_mut("body").expect("get_mut(body) failed") = serde_json::to_value(
            json!({
                "ok": false,
                "description": "Human-readable error description"
            })
            .to_string(),
        )
        .unwrap();
        let request_string = serde_json::to_string(&request_value)
            .expect("Failed to serialize request_value to string");

        let body = lambda_http::Body::from(request_string);

        let mut request = Request::new(body);
        request
            .headers_mut()
            .insert(CONTENT_TYPE, "application/json".parse().unwrap());
        let response = handler(request, Context::default())
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(
            response.body(),
            json!({"ok": false, "description": "Human-readable error description"})
                .into_response()
                .body()
        )
    }

    #[test]
    fn formats_name() {
        setup();

        use telegram_typings::User;
        let empty_user_fields = User {
            first_name: "".to_string(),
            last_name: None,
            username: None,
            id: 0,
            is_bot: false,
            language_code: None,
        };
        let user_all = User {
            first_name: "First".to_string(),
            last_name: Some("Last".to_string()),
            username: Some("daUser".to_string()),
            ..empty_user_fields.clone()
        };
        let user_all_but_last_name = User {
            first_name: "First".to_string(),
            username: Some("daUser".to_string()),
            ..empty_user_fields.clone()
        };
        let user_all_but_username = User {
            first_name: "First".to_string(),
            last_name: Some("Last".to_string()),
            ..empty_user_fields.clone()
        };
        let user_only_first_name = User {
            first_name: "First".to_string(),
            ..empty_user_fields.clone()
        };
        assert_eq!(format_name_for_user(&user_all), "@daUser (First Last)");
        assert_eq!(
            format_name_for_user(&user_all_but_last_name),
            "@daUser (First)"
        );
        assert_eq!(format_name_for_user(&user_all_but_username), "First Last");
        assert_eq!(format_name_for_user(&user_only_first_name), "First");
    }
}

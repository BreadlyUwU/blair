use std::sync::{Arc, LazyLock, RwLock};
use actix_web::HttpResponse;
use chrono::{DateTime, Local};
use crate::config::BLAIR;

const CACHE_TTL: chrono::Duration = chrono::Duration::minutes(5);

#[derive(Clone, Copy)]
enum STATUS {
    Online(DateTime<Local>),
    Offline(DateTime<Local>),
    Unchecked
}

pub async fn serve() -> HttpResponse {
    match &BLAIR.extra.fediverse {
        Some(fedi) => {
            match check(&fedi.health_endpoint).await {
                true => HttpResponse::Ok().content_type("application/json").body(r#"{"ok":"instance online"}"#),
                false => HttpResponse::ServiceUnavailable().append_header(("Retry-After", "300")).content_type("application/json").body(r#"{"ok":"instance offline"}"#),
            }
        },
        None => HttpResponse::InternalServerError().content_type("application/json").body(r#"{"error":"no instance set in B⋅LAIR's configuration"}"#),
    }
}

static CACHE_STATUS: LazyLock<Arc<RwLock<STATUS>>> = LazyLock::new(|| Arc::new(RwLock::new(STATUS::Unchecked)));
// Cache GTS's status in memory if the last health check has been done more than 5 minutes ago
// (or if no check has yet been done (aka: STATUS.status being None))
// then return the cached response.
async fn check(url: &String) -> bool {
    match read_cache() {
        STATUS::Online(t) | STATUS::Offline(t)
            if Local::now() - t < CACHE_TTL
            => matches!(read_cache(), STATUS::Online(..)),
        _ => ping(url).await,
    }
}

async fn ping(url: &String) -> bool {
    let online = match reqwest::Client::new()
        .get(url)
        .header(
            reqwest::header::USER_AGENT,
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        )
        .send()
        .await
    {
        Ok(res) => match res.status() {
            reqwest::StatusCode::OK => true,
            _                       => false,
        },
        Err(e)  => {
            log::warn!("{e}");
            false
        },
    };

    update_cache(online);
    online
}

fn read_cache() -> STATUS {
    *CACHE_STATUS.read().unwrap()
}

fn update_cache(new_status: bool) {
    let mut lock = CACHE_STATUS.write().unwrap();

    *lock = match new_status {
        true  => STATUS::Online(chrono::Local::now()),
        false => STATUS::Offline(chrono::Local::now()),
    };
}

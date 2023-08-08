use std::panic;

use js_sys::{Date, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use wee_alloc::WeeAlloc;

mod utils;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[doc(hidden)]
#[wasm_bindgen(start)]
pub fn _setup_console_error() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub async fn fetch_latest_commit(repo: &str, start: f64) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("https://api.github.com/repos/{repo}/commits?per_page=1");
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let response = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(response.is_instance_of::<Response>());
    let response: Response = response.dyn_into().unwrap();

    let json = JsFuture::from(response.json()?).await?;
    extract_first_commit_info(json, start)
}

fn extract_first_commit_info(json: JsValue, start: f64) -> Result<JsValue, JsValue> {
    let first = Reflect::get_u32(&json, 0)?;

    let sha = Reflect::get(&first, &JsValue::from_str("sha"))?;
    let commit = Reflect::get(&first, &JsValue::from_str("commit"))?;
    let author = Reflect::get(&commit, &JsValue::from_str("author"))?;
    let date = Reflect::get(&author, &JsValue::from_str("date"))?;

    let date = Date::new(&date).get_time();

    js_object! {
        sha: sha,
        days: utils::get_elapsed_days_since(date).into(),
        daysTotal: utils::get_elapsed_days_since(start).into(),
    }
}

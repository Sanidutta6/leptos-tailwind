use leptos::{logging, prelude::*, server::codee::string::FromToStringCodec};
use leptos_use::{UseCookieOptions, use_cookie, use_cookie_with_options};

/// Retrieves a cookie value by name
pub fn get_cookie(name: &str) -> Option<String> {
    let (cookie, _) = use_cookie::<String, FromToStringCodec>(name);
    cookie.get()
}

/// Sets a cookie value by name
pub fn set_cookie(name: &str, value: String) {
    let (_, set_cookie) = use_cookie_with_options::<String, FromToStringCodec>(
        name,
        UseCookieOptions::default().path("/"),
    );
    set_cookie.set(Some(value));
    logging::log!("Setting Cookie value: name: {}", name);
}

/// Deletes a cookie by name
pub fn delete_cookie(name: &str) {
    let (_, set_cookie) = use_cookie::<String, FromToStringCodec>(name);
    set_cookie.set(None);
}

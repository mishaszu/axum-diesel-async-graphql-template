use tower_cookies::{Cookie, Cookies};

use super::{crypt::token::Token, AUTH_TOKEN};

pub fn set_token_cookie(cookies: &Cookies, token: Token) {
    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);
}

pub fn remove_token_cookie(cookies: &Cookies) {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);
}

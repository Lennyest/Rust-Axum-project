use axum::{async_trait, RequestPartsExt};
use axum::extract::{FromRequestParts, State};
use axum::http::{Request};
use axum::http::request::Parts;
use axum::response::Response;
use axum::middleware::Next;
use tower_cookies::{Cookie, Cookies};
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};
use lazy_regex::regex_captures;
use crate::ctx::Ctx;
use crate::model::ModelController;

pub async fn mw_require_auth<B>(ctx: Result<Ctx>, req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:12} - mw_require_auth", "MIDDLEWARE");

    // Same as in mw_auth, but we don't need to extract the cookies, we already have them and we can just check if the cookie exists.
    ctx?;

    // TODO: Token component validation.

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver<B>(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // Compute Result<Ctx>
    let result_ctx = match auth_token.ok_or(Error::AuthFailNoAuthTokenCookie).and_then(parse_token) {
        Ok((user_id, _exp, _sign)) =>
        {
            // TODO: Token components validation
            Ok(Ctx::new(user_id))
        },
        Err(e) => Err(e),
    };

    // Remove the cookie if the token is invalid.
    if result_ctx.is_err() && !matches!(result_ctx.as_ref().unwrap_err(), Error::AuthFailNoAuthTokenCookie) {
        cookies.remove(Cookie::named(AUTH_TOKEN));
    }

    // Store the result in the request extensions.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

// Context Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> std::result::Result<Self, Self::Rejection> {
        println!("->> {:12} - CTX" , "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExtensions)?
            .clone()
    }
}


// Parse a token of format 'user-[user-id].[expiration].[signature]'
// Returns (user-id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {

    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}

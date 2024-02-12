use axum::{routing::get, Router};

use crate::handler::auth_handler::AuthHandler;

pub struct AuthRouter;

impl AuthRouter {
    pub fn authrouter_init() -> Router {
        Router::new().route("/auth", get(AuthHandler::auth_handler()))
    }

    fn idrouter_init() -> Router {
        Router::new().route("/id", method_router)
    }
}

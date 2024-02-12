pub mod auth_router;

use axum::Router;

use self::auth_router::AuthRouter;

pub fn router_init() -> Router {
    Router::new().merge(AuthRouter::authrouter_init())
}

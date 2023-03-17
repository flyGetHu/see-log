use salvo::logging::Logger;
use salvo::Router;

pub mod log_route;
pub mod health_route;

pub fn init_route() -> Router {
    Router::new()
        .hoop(Logger)
        .push(Router::with_path("/health").get(health_route::health))
        .push(Router::with_path("/see/log").get(log_route::see_log))
}

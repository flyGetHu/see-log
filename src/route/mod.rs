use salvo::logging::Logger;
use salvo::Router;

pub mod health_route;
pub mod log_route;

pub fn init_route() -> Router {
    Router::new()
        .hoop(Logger::new())
        .push(Router::with_path("/health").get(health_route::health))
        .push(Router::with_path("/see/log").get(log_route::see_log))
}

use salvo::Router;

pub mod log_route;

pub fn init_route() -> Router {
    Router::new().push(Router::with_path("/see/log").get(log_route::see_log))
}

use salvo::logging::Logger;
use salvo::prelude::{OpenApi, SwaggerUi};
use salvo::Router;

pub mod health_route;
pub mod log_route;

pub fn init_route() -> Router {
    let router = Router::new()
        .hoop(Logger::new())
        .push(Router::with_path("/health").get(health_route::health))
        .push(Router::with_path("/see/log").get(log_route::see_log));
    let doc = OpenApi::new("test api", "0.0.1").merge_router(&router);
    Router::new()
        .push(router)
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}

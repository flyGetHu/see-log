/// Initializes the routes for the application.
///
/// This function creates a new router and adds two routes to it:
/// - "/health" which returns the health status of the application
/// - "/see/log" which returns the logs of the application
///
/// It also adds OpenAPI documentation for the router and Swagger UI for the documentation.
///
/// # Returns
///
/// A `Router` instance with the defined routes and documentation.
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
    // swagger
    let doc = OpenApi::new("open api", "0.0.1").merge_router(&router);
    Router::new()
        .push(router)
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}

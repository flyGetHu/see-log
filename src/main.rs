use salvo::prelude::*;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(Router::with_path("/see/log").get(see_log::route::log_route::see_log));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await;
}

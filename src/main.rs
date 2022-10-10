use salvo::Catcher;
use salvo::prelude::*;
use see_log::route::*;
use see_log::handle::*;



#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(Router::with_path("/see/log").get(log_route::see_log));
    let catchers:Vec<Box<dyn Catcher>>=vec![Box::new(catcher_handle::Handle404)];
    let service=Service::new(router).with_catchers(catchers);
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(service)
        .await;
}

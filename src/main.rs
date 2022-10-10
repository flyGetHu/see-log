use salvo::prelude::*;

use see_log::handle::*;
use see_log::route::*;

#[tokio::main]
async fn main() {
    //挂载路由
    let service = Service::new(init_route())
        .with_catchers(inti_catcher());
    let tcp_listener = TcpListener::bind("0.0.0.0:3000");
    Server::new(tcp_listener)
        .serve(service)
        .await;
}

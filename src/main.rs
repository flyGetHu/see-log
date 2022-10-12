use salvo::__private::tracing;
use salvo::prelude::*;

use see_log::handle::*;
use see_log::route::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    //挂载路由
    let service = Service::new(init_route()).with_catchers(inti_catcher());
    let address = "0.0.0.0:3000";
    let tcp_listener = TcpListener::bind(address);
    tracing::info!("项目启动成功:{address}");
    Server::new(tcp_listener).serve(service).await;
}

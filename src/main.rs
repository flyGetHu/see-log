use ferris_says::say;
use salvo::prelude::*;
use std::io::{stdout, BufWriter};

use see_log::handle::*;
use see_log::route::*;

#[tokio::main]
async fn main() {
    //初始化日志记录
    tracing_subscriber::fmt().init();

    let address = "0.0.0.0:3000";
    let tcp_listener = TcpListener::bind(address);

    //打印启动日志
    {
        let message = format!("项目启动成功:{address}");
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout.lock());
        say(message.as_str(), message.chars().count() + 6, &mut writer).unwrap();
    }

    //初始化路由 启动webserver
    let service = Service::new(init_route()).with_catchers(inti_catcher());
    Server::new(tcp_listener).serve(service).await;
}

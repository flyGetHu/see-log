use std::io::{BufWriter, stdout};

use ferris_says::say;
use salvo::catcher::Catcher;
use salvo::prelude::*;

use see_log::route::*;

#[tokio::main]
async fn main() {
    //初始化日志记录
    tracing_subscriber::fmt().init();

    let address = "0.0.0.0:3000";
    let tcp_listener = TcpListener::new(address).bind().await;

    //打印启动日志
    {
        let message = format!("项目启动成功:{address}");
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout.lock());
        say(message.as_str(), message.chars().count() + 6, &mut writer).unwrap();
    }

    //初始化路由 启动webserver
    let service = Service::new(init_route()).catcher(Catcher::default().hoop(handle404));
    Server::new(tcp_listener).serve(service).await;
}

#[handler]
async fn handle404(_req: &Request, _depot: &Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        res.render("Custom 404 Error Page");
        ctrl.skip_rest();
    }
}
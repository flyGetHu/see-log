use std::io::{stdout, BufWriter};

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
    let service = Service::new(init_route()).with_catcher(Catcher::default().hoop(catcher_handle));
    Server::new(tcp_listener).serve(service).await;
}

// 统一处理异常
#[handler]
async fn catcher_handle(
    &self,
    _req: &Request,
    _depot: &Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    match res.status_code() {
        None => {
            res.set_status_code(StatusCode::NOT_FOUND);
        }
        Some(status_code) => match status_code {
            StatusCode::NOT_FOUND => {
                res.render("404");
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                res.render("系统错误,请联系管理员");
            }
            _ => {
                res.render("系统错误,请联系管理员");
            }
        },
    }
    ctrl.skip_rest();
}

use std::collections::HashMap;

use salvo::{fs::NamedFile, prelude::*};
use salvo::http::HeaderMap;
use salvo::routing::get;

#[handler]
async fn hello_world(
    _req: &mut Request,
    _depot: &mut Depot,
    res: &mut Response,
    _ctrl: &mut FlowCtrl,
) {
    res.render("Hello world");
}

#[handler]
async fn see_log(req: &mut Request, res: &mut Response) {
    let header = HeaderMap::new();
    NamedFile::send_file(
        "C:\\Users\\97078\\Desktop\\fsdownload\\access.log",
        &header,
        res,
    )
        .await;
}

#[tokio::main]
async fn main() {
    let router = Router::new().get(hello_world)
        .push(Router::with_path("/see/log").get(see_log));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await;
}

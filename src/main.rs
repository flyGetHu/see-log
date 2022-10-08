use std::fs::File;
use std::io::{BufRead, BufReader};

use salvo::prelude::*;

#[handler]
async fn see_log(req: &mut Request, res: &mut Response) {
    let count = req.query("count").unwrap_or_else(|| 1024);
    // let path = "C:\\Users\\97078\\Desktop\\fsdownload\\access.log";
    let path = "/home/work/express-app/express-app.log";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut res_list = Vec::with_capacity(1024);
    for line in lines {
        let line_res = line.unwrap();
        res_list.push(line_res)
    }
    let start = res_list.len() - count;
    let res_list_final = &res_list[start..];
    let mut res_str = String::new();
    for line in res_list_final {
        res_str += &format!("{}\n", line);
    }
    res.render(format!("{}", res_str))
}

#[tokio::main]
async fn main() {
    let router = Router::new().push(Router::with_path("/see/log").get(see_log));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router)
        .await;
}

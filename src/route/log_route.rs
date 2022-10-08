use salvo::{handler, Request, Response};

#[handler]
pub async fn see_log(req: &mut Request, res: &mut Response) {
    let count = req.query("count").unwrap_or_else(|| 1024);

    let model_name = req.query("model").unwrap_or_else(|| "express");
    // let mut file_path = "C:\\Users\\97078\\Desktop\\fsdownload\\access.log";
    let mut file_path = "/home/work/express-app/express-app.log";
    if model_name == "express" {
        // file_path = "C:\\Users\\97078\\Desktop\\fsdownload\\access.log";
        file_path = "/home/work/express-app/express-app.log";
    }
    let res_str = crate::service::log_service::load_log_file(file_path, count);
    res.render(format!("{}", res_str))
}

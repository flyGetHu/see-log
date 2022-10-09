use salvo::{handler, Request, Response};

#[handler]
pub async fn see_log(req: &mut Request, res: &mut Response) {
    let count = req.query("count").unwrap_or_else(|| 1024);
    // let mut file_path = "C:\\Users\\97078\\Desktop\\fsdownload\\access.log";
    let mode_name = req.query("model").unwrap_or_else(|| "express");
    let log_level = req.query("logLevel").unwrap_or_else(|| "info");
    let res_str = crate::service::log_service::load_log_file(mode_name, log_level, count);
    res.render(format!("{}", res_str))
}

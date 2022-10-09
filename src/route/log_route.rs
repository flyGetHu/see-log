use salvo::{handler, Request, Response};

#[handler]
pub async fn see_log(req: &mut Request, res: &mut Response) {
    let count = req.query("count").unwrap_or_else(|| 1024);
    // let mut file_path = "C:\\Users\\97078\\Desktop\\fsdownload\\access.log";
    let mode_name = req.query("model").unwrap_or_else(|| "express").to_string();
    let mut file_path = "";
    if mode_name == "express" {
        file_path = "/home/work/express-app/express-app.log"
    } else if mode_name == "admin-oa" {
        file_path = "/home/work/admin-oa/admin-oa.log"
    }else{
      
    }
    let res_str = crate::service::log_service::load_log_file(file_path, count);
    res.render(format!("{}", res_str))
}

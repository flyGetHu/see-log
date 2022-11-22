use crate::service::*;
use salvo::http::StatusError;
use salvo::{handler, Request, Response};

use crate::entity;

//最大可以查看的日志行数
const MAX_LINE_SIZE: usize = 1024;

#[handler]
pub async fn see_log(req: &mut Request, res: &mut Response) {
    let mut count = req.query("count").unwrap_or_else(|| MAX_LINE_SIZE / 2);
    if count > MAX_LINE_SIZE {
        count = MAX_LINE_SIZE
    }
    let file_path_query = req.query("file_path");
    match file_path_query {
        None => {
            res.set_status_error(StatusError::internal_server_error().with_detail("file_path必传"));
        }
        Some(file_path) => {
            let result = entity::log_file::LogFile { file_path, count }.load_log_file();
            match result {
                Ok(data) => res.render(format!("{}", data)),
                Err(err) => {
                    res.set_status_error(StatusError::internal_server_error().with_detail(err))
                }
            }
        }
    }
}

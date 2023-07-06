use salvo::{handler, Request, Response};
use salvo::http::StatusError;

use entity::log_file::LogFile;

use crate::entity;

//最大可以查看的日志行数
const MAX_LINE_SIZE: usize = 1024;

#[handler]
pub async fn see_log(req: &mut Request, res: &mut Response) {
    let count = req
        .query("count")
        .unwrap_or(MAX_LINE_SIZE / 2)
        .min(MAX_LINE_SIZE);
    let file_path_query = req.query("file_path");
    if let Some(file_path) = file_path_query {
        let result = LogFile { file_path, count }.load_log_file();
        match result {
            Ok(data) => res.render(format!("{}", data)),
            Err(err) => res.render(StatusError::internal_server_error().brief(err)),
        }
    } else {
        res.render(StatusError::internal_server_error().brief("file_path必传"));
    }
}

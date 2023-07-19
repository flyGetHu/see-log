use salvo::http::StatusError;
use salvo::oapi::extract::QueryParam;
use salvo::{endpoint, Request, Response};

use entity::log_file::LogFile;

use crate::entity;

//最大可以查看的日志行数
const MAX_LINE_SIZE: usize = 1024;

#[endpoint]
pub async fn see_log(
    count: QueryParam<usize, true>,
    file_path: QueryParam<String, true>,
    res: &mut Response,
) {
    let count = count.min(MAX_LINE_SIZE);
    if file_path.len() == 0 {
        res.render(StatusError::internal_server_error().brief("file_path必传"));
        return;
    }
    let file_path = file_path.into_inner();
    let result = LogFile { file_path, count }.load_log_file();
    match result {
        Ok(data) => res.render(format!("{}", data)),
        Err(err) => res.render(StatusError::internal_server_error().brief(err)),
    }
}

use crate::service::*;
use salvo::http::StatusError;
use salvo::{handler, Request, Response};

use crate::entity;

#[handler]
pub async fn see_log(req: &mut Request, res: &mut Response) {
    let count = req.query("count").unwrap_or_else(|| 1024);
    let file_path_query = req.query("file_path");
    match file_path_query {
        None => {
            res.set_status_error(StatusError::internal_server_error().with_detail("file_path必传"));
        }
        Some(file_path) => {
            let project_model = entity::LogParam { file_path, count };
            let result = log_service::load_log_file(project_model);
            match result {
                Ok(data) => res.render(format!("{}", data)),
                Err(err) => {
                    res.set_status_error(StatusError::internal_server_error().with_detail(err))
                }
            }
        }
    }
}

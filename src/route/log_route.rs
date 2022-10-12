use crate::service::*;
use salvo::http::StatusError;
use salvo::{handler, Request, Response};

use crate::entity;

#[handler]
pub async fn see_log(req: &mut Request, res: &mut Response) {
    let count = req.query("count").unwrap_or_else(|| 1024);
    let mode_name = req
        .query("model")
        .unwrap_or_else(|| String::from("express"));
    let log_level = req
        .query("logLevel")
        .unwrap_or_else(|| String::from("info"));
    let project_model = entity::ProjectModel {
        mode_name,
        log_level,
        count,
    };
    let result = log_service::load_log_file(project_model);
    match result {
        Ok(data) => res.render(format!("{}", data)),
        Err(err) => res.set_status_error(StatusError::internal_server_error().with_detail(err)),
    }
}

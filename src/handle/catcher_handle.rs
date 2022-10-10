use salvo::{Catcher, Depot, Request, Response};
use salvo::prelude::StatusCode;

pub struct Handle404;

/// 处理http状态码异常情况
/// 404 503
impl Catcher for Handle404 {
    fn catch(&self, _req: &Request, _depot: &Depot, res: &mut Response) -> bool {
        if let Some(StatusCode::NOT_FOUND) = res.status_code() {
            res.render("Custom 404 Error Page");
            true
        } else if Some(StatusCode::SERVICE_UNAVAILABLE) = res.status_code() {
            res.render("503 Service Unavailable");
            true
        } else {
            false
        }
    }
}
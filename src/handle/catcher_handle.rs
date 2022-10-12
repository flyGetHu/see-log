use salvo::prelude::StatusCode;
use salvo::{Catcher, Depot, Request, Response};

pub struct CatcherHandle;

/// 处理http状态码异常情况
/// 404 503
impl Catcher for CatcherHandle {
    fn catch(&self, _req: &Request, _depot: &Depot, res: &mut Response) -> bool {
        if let Some(StatusCode::NOT_FOUND) = res.status_code() {
            res.render("Custom 404 Error Page");
            true
        } else if Some(StatusCode::SERVICE_UNAVAILABLE) == res.status_code() {
            res.render("503 Service Unavailable");
            true
        } else {
            false
        }
    }
}

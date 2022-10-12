use salvo::prelude::StatusCode;
use salvo::{Catcher, Depot, Request, Response};

pub struct CatcherHandle;

/// 处理http状态码异常情况
/// 404 503
impl Catcher for CatcherHandle {
    fn catch(&self, req: &Request, _depot: &Depot, res: &mut Response) -> bool {
        let status_code = res.status_code();
        return match status_code {
            None => false,
            Some(code) => {
                tracing::error!("客户端异常请求{:?}", req);
                return match code {
                    StatusCode::NOT_FOUND => {
                        res.render("Custom 404 Error Page");
                        true
                    }
                    StatusCode::SERVICE_UNAVAILABLE => {
                        res.render("503 Service Unavailable");
                        true
                    }
                    _status_code => false,
                };
            }
        };
    }
}

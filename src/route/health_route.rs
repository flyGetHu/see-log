use salvo::{handler, Request, Response};

#[handler]
pub async fn health(_: &mut Request, res: &mut Response) {
    res.render("success")
}

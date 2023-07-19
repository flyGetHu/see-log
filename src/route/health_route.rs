use salvo::{endpoint, Request, Response};

#[endpoint]
pub async fn health(_: &mut Request, res: &mut Response) {
    res.render("success")
}

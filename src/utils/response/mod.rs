use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};

pub enum ApiResponse<'a, T: IntoResponse> {
    Ok(ResponseBody<'a, T>),
    Err(ResponseBody<'a, T>),
}

pub struct ResponseBody<'a, T: IntoResponse> {
    code: StatusCode,
    msg: Option<&'a str>,
    data: Option<T>,
}

impl<'a, T: IntoResponse> IntoResponse for ResponseBody<'a, T> {
    fn into_response(self) -> axum::response::Response {
        Response::builder()
            .status(StatusCode::from(self.code))
            .body(Body::from("test"))
            .unwrap_or_else(op)
    }
}

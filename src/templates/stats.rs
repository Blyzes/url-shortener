use askama::Template;
use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

#[derive(Template)]
#[template(path = "stats.html")]
pub struct StatsTemplate {
    pub key: String,
    pub url: String,
    pub clicks: i64,
}

// 手动实现 IntoResponse
impl IntoResponse for StatsTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => {
                (StatusCode::OK, [(header::CONTENT_TYPE, "text/html")], html).into_response()
            }
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Template rendering error".to_string(),
            )
                .into_response(),
        }
    }
}

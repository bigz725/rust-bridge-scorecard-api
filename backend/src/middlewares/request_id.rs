use axum::{body::Body, extract::Request, http::Response, middleware::Next};

#[derive(Debug, Clone)]
pub struct RequestId(pub uuid::Uuid);

#[tracing::instrument(skip(request, next), level="trace")]
pub async fn add_session_id(mut request: Request, next: Next) -> Response<Body> {
    let uuid = uuid::Uuid::now_v7();
    let session_id = RequestId(uuid);
    request.extensions_mut().insert(session_id);
    next.run(request).await
}

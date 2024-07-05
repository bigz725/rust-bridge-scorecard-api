use crate::{
    auth::logout::{logout, LogoutError}, middlewares::auth::{lookup_user::lookup_user_from_token, verify_jwt::get_claims_from_auth_token}, models::user::User, state::AppState
};
use axum::{body::Body, debug_handler, extract::{Json, Request, State}, middleware, response::{IntoResponse, Response}, routing::post, Router};
use serde_json::{json, Value};

pub fn routes(state: &AppState) -> Router<AppState> {
    let get_claims_layer = middleware::from_fn_with_state(state.clone(), get_claims_from_auth_token);
    let lookup_user_layer = middleware::from_fn_with_state(state.clone(), lookup_user_from_token);
    Router::new().route("/api/auth/logout", post(handle_logout))
                 .route("/api/auth/signout", post(handle_logout))
                 .route_layer(lookup_user_layer)
                 .route_layer(get_claims_layer)
}

#[tracing::instrument(skip(db, request,))]
#[debug_handler]
async fn handle_logout(
        State(AppState {
        mongodb_client: db,
        keys: _,
    }): State<AppState>,    
        request: Request,) -> Result<Json<Value>, LogoutError> {
    let mut user = request.extensions().get::<User>().unwrap().clone();
    logout(&db, &mut user).await?;
    Ok(Json(json!({"message": "Successfully logged out"})))
}

impl IntoResponse for LogoutError {
    fn into_response(self) -> Response<Body> {
        todo!()
    }
}
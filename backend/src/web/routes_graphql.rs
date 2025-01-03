use async_graphql::{extensions::Tracing, http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{body::Body, debug_handler, extract::{Request, State}, middleware::Next, response::{self, IntoResponse, Response}, routing::get, Extension, Router};
use axum::middleware;
use crate::{auth::{jwt::Claims, login::LoginError}, graphql::user::{Mutation, Query}, middlewares::auth::verify_jwt::{get_claims, BearerToken}, models::user::{find_user, User, UserError}, state::AppState};



#[tracing::instrument(target="routes")]
async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .finish(),
    )
}
#[tracing::instrument(skip(db, keys, maybe_user, token, req, diesel))]
#[debug_handler]
async fn graphql_handler(
    State(AppState{db_conn: db, diesel_conn: diesel, keys}): State<AppState>, 
    Extension(maybe_user): Extension<Option<User>>,
    token: Option<BearerToken>, 
    req: GraphQLRequest) -> GraphQLResponse {
    let req = req.into_inner();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db.clone())
        .data(diesel.clone())
        .data(keys.clone())
        .data(maybe_user.clone())
        .data(token)
        .extension(Tracing)
        .finish();
    schema.execute(req).await.into()

}

pub fn routes(state: &AppState) -> Router<AppState> {
    let get_claims_from_optional_auth_token_layer = 
        middleware::from_fn_with_state(state.clone(), get_claims_from_optional_auth_token);
    let lookup_user_layer = 
        middleware::from_fn_with_state(state.clone(), lookup_user_from_token);
    Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .route_layer(lookup_user_layer)
        .route_layer(get_claims_from_optional_auth_token_layer)
}

// Region: middleware


#[tracing::instrument(skip(auth_token, keys, request, next))]
async fn get_claims_from_optional_auth_token(
    auth_token: Option<BearerToken>,
    State(AppState{db_conn: _, diesel_conn: _, keys}): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response<Body> {
    if let Some(token) = auth_token {
        
        let bearer_token = token.0.replace("Bearer ", "");
        let decoding_key = keys.decoding;
        match get_claims(&bearer_token, &decoding_key) {
            Ok(claims) => {
                request.extensions_mut().insert::<Option<Claims>>(Some(claims.clone()));
                next.run(request).await
            }
            Err(e) => {
                request.extensions_mut().insert::<Option<Claims>>(None);
                tracing::error!("Error decoding JWT: {:?}", e);
                e.into_response()
            }
        }
    }
    else {
       request.extensions_mut().insert::<Option<Claims>>(None);
       next.run(request).await
    }
}

#[tracing::instrument(skip(claims, diesel, request, next))]
pub async fn lookup_user_from_token(
    Extension(claims): Extension<Option<Claims>>,
    State(AppState{ db_conn: _, diesel_conn: diesel, keys: _}): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response<Body> {
    if let Some(claims) = claims {
        let result = find_user(&diesel, Some(&claims.id), None, None, Some(&claims.salt))
            .await
            .map_err(LoginError::from);
        let users = match result {
            Ok(users) => users,
            Err(err) => {
                tracing::error!("Error looking up user with id: {} and salt: {}", &claims.id, &claims.salt);
                request.extensions_mut().insert::<Option<User>>(None);
                return err.into_response();
            }
        };
        let user = users.first();
        match user {
            Some(user) => {
                tracing::info!("User {} successfully looked up", user.username.clone());
                request.extensions_mut().insert(Some(user.to_owned()));
                next.run(request).await
            }
            None => {
                tracing::error!(
                    "No user found with id: {} and salt: {}",
                    &claims.id,
                    &claims.salt
                );
                request.extensions_mut().insert::<Option<User>>(None);
                LoginError::from(UserError::UserNotFound).into_response()
            }
        }
    }
    else {
        request.extensions_mut().insert::<Option<User>>(None);
        next.run(request).await
    }
}
// end region

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

use crate::database::UserDatabase;
use crate::entities::User;

pub async fn register_user(Json(user): Json<User>) -> impl IntoResponse {
    if user.age.is_some() && user.name.is_some() {
        UserDatabase::save(&user).await;

        StatusCode::CREATED
    } else {
        StatusCode::BAD_REQUEST
    }
}

pub async fn find_user(Path(id): Path<u64>) -> Result<(StatusCode, Json<User>), StatusCode> {
    if let Ok(user) = UserDatabase::find_by_id(id).await {
        return Ok((StatusCode::OK, Json(user)));
    }
    Err(StatusCode::OK)
}

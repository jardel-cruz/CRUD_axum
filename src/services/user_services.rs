use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

use crate::database::UserDatabase;
use crate::entities::User;

pub async fn register_user(Json(user): Json<User>) -> impl IntoResponse {
    let valid = {
        if user.age.is_some()
            && user.name.is_some()
            && user.email.is_some()
            && user.password.is_some()
        {
            true
        } else {
            false
        }
    };

    if valid {
        UserDatabase::save(&user).await;

        StatusCode::CREATED
    } else {
        StatusCode::BAD_REQUEST
    }
}

pub async fn find_user(Path(id): Path<u64>) -> Result<impl IntoResponse, StatusCode> {
    if let Some(user) = UserDatabase::find_by_id(id).await {
        return Ok((StatusCode::OK, Json(user)));
    }
    Err(StatusCode::OK)
}

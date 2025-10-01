use actix_web::{web, HttpResponse, HttpRequest, HttpMessage};
use sea_orm::DatabaseConnection;
use crate::schemas::auth::Claims;
use crate::service::labeler::LabelerService;

pub async fn get_groups(
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, actix_web::Error> {
    // Extract user info from request extensions (set by middleware)
    let claims = req.extensions().get::<Claims>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("User not authenticated"))?
        .clone();

    let labeler_id = claims.user_id;
    
    match LabelerService::get_labeler_groups(&db, labeler_id).await {
        Ok(response) => {
            Ok(HttpResponse::Ok().json(response.data.unwrap().groups))
        }
        Err(e) => {
            eprintln!("Error fetching groups for labeler {}: {}", labeler_id, e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e
            })))
        }
    }
}

pub async fn get_group_images(
    req: HttpRequest,
    path: web::Path<i32>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, actix_web::Error> {
    // Extract user info from request extensions (set by middleware)
    let claims = req.extensions().get::<Claims>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("User not authenticated"))?
        .clone();

    let labeler_id = claims.user_id;
    let group_id = path.into_inner();
    
    match LabelerService::get_group_images(&db, labeler_id, group_id).await {
        Ok(response) => {
            Ok(HttpResponse::Ok().json(response.data.unwrap().images))
        }
        Err(e) => {
            if e.contains("not authorized") {
                Ok(HttpResponse::Forbidden().json(serde_json::json!({
                    "error": e
                })))
            } else {
                eprintln!("Error fetching images for group {}: {}", group_id, e);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": e
                })))
            }
        }
    }
}

use actix_web::{web, HttpResponse, HttpRequest, HttpMessage};
use sea_orm::DatabaseConnection;
use crate::schemas::auth::Claims;
use crate::service::labeler::LabelerService;

pub async fn get_labeler_image_details(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, actix_web::Error> {
    // Extract user info from request extensions (set by middleware)
    let claims = req.extensions().get::<Claims>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("User not authenticated"))?
        .clone();

    let labeler_id = claims.user_id;
    let (group_id, image_id) = path.into_inner();
    
    match LabelerService::get_image_details(&db, labeler_id, group_id, image_id).await {
        Ok(response) => {
            Ok(HttpResponse::Ok().json(response.data.unwrap()))
        }
        Err(e) => {
            if e.contains("not authorized") {
                Ok(HttpResponse::Forbidden().json(serde_json::json!({
                    "success": false,
                    "error": e,
                    "data": null
                })))
            } else {
                eprintln!("Error fetching image details: {}", e);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "error": e,
                    "data": null
                })))
            }
        }
    }
}

pub async fn update_image_tags(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    db: web::Data<DatabaseConnection>,
    update_request: web::Json<crate::schemas::labeler::UpdateImageTagsRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    // Extract user info from request extensions (set by middleware)
    let claims = req.extensions().get::<Claims>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("User not authenticated"))?
        .clone();

    let labeler_id = claims.user_id;
    let (group_id, image_id) = path.into_inner();
    
    match LabelerService::update_image_tags(&db, labeler_id, group_id, image_id, update_request.into_inner()).await {
        Ok(response) => {
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            if e.contains("not authorized") {
                Ok(HttpResponse::Forbidden().json(serde_json::json!({
                    "success": false,
                    "error": e,
                    "data": null
                })))
            } else {
                eprintln!("Error updating image tags: {}", e);
                Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "error": e,
                    "data": null
                })))
            }
        }
    }
}

pub async fn suggest_tags(
    req: HttpRequest,
    path: web::Path<i32>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, actix_web::Error> {
    // Extract user info from request extensions (set by middleware)
    let claims = req.extensions().get::<Claims>()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("User not authenticated"))?
        .clone();

    let labeler_id = claims.user_id;
    let image_id = path.into_inner();
    
    match LabelerService::suggest_tags(&db, labeler_id, image_id).await {
        Ok(response) => {
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            eprintln!("Error suggesting tags: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": e,
                "data": null
            })))
        }
    }
}

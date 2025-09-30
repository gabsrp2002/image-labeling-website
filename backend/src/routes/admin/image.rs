use actix_web::{web, HttpResponse, Result};
use sea_orm::DatabaseConnection;
use crate::schemas::admin::{ApiResponse, UploadImageRequest};
use crate::service::admin::AdminService;

pub async fn upload_image(
    db: web::Data<DatabaseConnection>,
    request: web::Json<UploadImageRequest>,
) -> Result<HttpResponse> {
    match AdminService::upload_image(&db, request.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: e,
            data: None,
        })),
    }
}

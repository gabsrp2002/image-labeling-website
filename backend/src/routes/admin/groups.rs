use actix_web::{web, HttpResponse, Result};
use sea_orm::DatabaseConnection;
use crate::schemas::admin::ApiResponse;
use crate::service::admin::AdminService;

pub async fn list_groups(
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse> {
    match AdminService::list_groups(&db).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: e,
            data: None,
        })),
    }
}

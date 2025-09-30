use actix_web::{web, HttpResponse, Result};
use sea_orm::DatabaseConnection;
use crate::schemas::admin::{ApiResponse, CreateGroupRequest};
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

pub async fn create_group(
    db: web::Data<DatabaseConnection>,
    request: web::Json<CreateGroupRequest>,
) -> Result<HttpResponse> {
    match AdminService::create_group(&db, request.into_inner()).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: e,
            data: None,
        })),
    }
}

pub async fn delete_group(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let group_id = path.into_inner();
    match AdminService::delete_group(&db, group_id).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: e,
            data: None,
        })),
    }
}

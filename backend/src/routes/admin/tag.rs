use actix_web::{web, HttpResponse, Result};
use sea_orm::DatabaseConnection;
use crate::schemas::admin::{ApiResponse, CreateTagRequest, UpdateTagRequest};
use crate::service::admin::AdminService;

pub async fn create_tag(
    db: web::Data<DatabaseConnection>,
    request: web::Json<CreateTagRequest>,
) -> Result<HttpResponse> {
    match AdminService::create_tag(&db, request.into_inner()).await {
        Ok(response) => {
            if response.success {
                Ok(HttpResponse::Created().json(response))
            } else {
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: e,
            data: None,
        })),
    }
}

pub async fn get_tag(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let tag_id = path.into_inner();
    
    match AdminService::get_tag(&db, tag_id).await {
        Ok(response) => {
            if response.success {
                Ok(HttpResponse::Ok().json(response))
            } else {
                Ok(HttpResponse::NotFound().json(response))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: e,
            data: None,
        })),
    }
}

pub async fn list_tags_by_group(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let group_id = path.into_inner();
    
    match AdminService::list_tags_by_group(&db, group_id).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: e,
            data: None,
        })),
    }
}

pub async fn update_tag(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
    request: web::Json<UpdateTagRequest>,
) -> Result<HttpResponse> {
    let tag_id = path.into_inner();
    
    match AdminService::update_tag(&db, tag_id, request.into_inner()).await {
        Ok(response) => {
            if response.success {
                Ok(HttpResponse::Ok().json(response))
            } else {
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: e,
            data: None,
        })),
    }
}

pub async fn delete_tag(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let tag_id = path.into_inner();
    
    match AdminService::delete_tag(&db, tag_id).await {
        Ok(response) => {
            if response.success {
                Ok(HttpResponse::Ok().json(response))
            } else {
                Ok(HttpResponse::NotFound().json(response))
            }
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: e,
            data: None,
        })),
    }
}

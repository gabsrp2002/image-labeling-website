use actix_web::{web, HttpResponse, Result};
use sea_orm::DatabaseConnection;
use crate::schemas::admin::{
    CreateLabelerRequest, UpdateLabelerRequest, ApiResponse
};
use crate::service::admin::AdminService;

pub async fn create_labeler(
    db: web::Data<DatabaseConnection>,
    request: web::Json<CreateLabelerRequest>,
) -> Result<HttpResponse> {
    match AdminService::create_labeler(&db, request.into_inner()).await {
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

pub async fn get_labeler(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let labeler_id = path.into_inner();
    
    match AdminService::get_labeler(&db, labeler_id).await {
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

pub async fn list_labelers(
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse> {
    match AdminService::list_labelers(&db).await {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: e,
            data: None,
        })),
    }
}

pub async fn update_labeler(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
    request: web::Json<UpdateLabelerRequest>,
) -> Result<HttpResponse> {
    let labeler_id = path.into_inner();
    
    match AdminService::update_labeler(&db, labeler_id, request.into_inner()).await {
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

pub async fn delete_labeler(
    db: web::Data<DatabaseConnection>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let labeler_id = path.into_inner();
    
    match AdminService::delete_labeler(&db, labeler_id).await {
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

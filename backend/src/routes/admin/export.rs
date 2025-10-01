use actix_web::{web, HttpResponse, Result};
use crate::schemas::export::ExportResponse;
use crate::service::export::ExportService;

pub async fn bulk_export(
    db: web::Data<sea_orm::DatabaseConnection>,
) -> Result<HttpResponse> {
    match ExportService::bulk_export(&db).await {
        Ok(export_data) => {
            Ok(HttpResponse::Ok().json(ExportResponse {
                success: true,
                message: "Bulk export completed successfully".to_string(),
                data: Some(serde_json::to_value(export_data).unwrap_or(serde_json::Value::Object(serde_json::Map::new()))),
            }))
        }
        Err(error_message) => {
            Ok(HttpResponse::InternalServerError().json(ExportResponse {
                success: false,
                message: error_message,
                data: None,
            }))
        }
    }
}
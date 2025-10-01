use serde::Serialize;

#[derive(Serialize)]
pub struct ExportResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct ExportData {
    #[serde(flatten)]
    pub groups: std::collections::HashMap<String, GroupData>,
}

#[derive(Serialize)]
pub struct GroupData {
    #[serde(flatten)]
    pub images: std::collections::HashMap<String, ImageData>,
}

#[derive(Serialize)]
pub struct ImageData {
    pub filename: String,
    pub filetype: String,
    pub base64: String,
    pub uploaded_at: String,
    pub final_tags: Vec<String>,
    pub tag_statistics: Vec<TagStatistic>,
    pub has_admin_override: bool,
}

#[derive(Serialize)]
pub struct TagStatistic {
    pub tag_id: i32,
    pub tag_name: String,
    pub percentage: f64,
    pub count: i32,
    pub total_labelers: i32,
}

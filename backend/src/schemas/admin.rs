use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateLabelerRequest {
    pub username: String,
    pub password: String,
    pub group_ids: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLabelerRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub group_ids: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LabelerResponse {
    pub id: i32,
    pub username: String,
    pub group_ids: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct GroupResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GroupListResponse {
    pub groups: Vec<GroupResponse>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct LabelerListResponse {
    pub labelers: Vec<LabelerResponse>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct SimpleLabelerResponse {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct TagResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ImageResponse {
    pub id: i32,
    pub filename: String,
    pub filetype: String,
    pub uploaded_at: String,
}

#[derive(Debug, Serialize)]
pub struct GroupDetailResponse {
    pub group: GroupResponse,
    pub labelers: Vec<SimpleLabelerResponse>,
    pub tags: Vec<TagResponse>,
    pub images: Vec<ImageResponse>,
}

#[derive(Debug, Deserialize)]
pub struct UploadImageRequest {
    pub filename: String,
    pub filetype: String,
    pub base64_data: String,
    pub group_id: i32,
}

#[derive(Debug, Serialize)]
pub struct ImageUploadResponse {
    pub id: i32,
    pub filename: String,
    pub filetype: String,
    pub uploaded_at: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

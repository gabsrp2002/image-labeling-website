use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct GroupResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ImageResponse {
    pub id: i32,
    pub filename: String,
    pub status: String, // "done" or "pending"
    pub base64_data: String,
    pub filetype: String,
}

#[derive(Debug, Serialize)]
pub struct GroupListResponse {
    pub groups: Vec<GroupResponse>,
}

#[derive(Debug, Serialize)]
pub struct ImageListResponse {
    pub images: Vec<ImageResponse>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize)]
pub struct TagResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ImageDetailResponse {
    pub image: ImageResponse,
    pub group_tags: Vec<TagResponse>,
    pub current_tags: Vec<TagResponse>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateImageTagsRequest {
    pub tag_ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SuggestTagsRequest {
    pub ignored_tag_ids: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct SuggestTagsResponse {
    pub suggested_tags: Vec<String>,
}

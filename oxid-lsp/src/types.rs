pub struct Request {
    id: i64,
    method: String,
    params: Option<serde_json::Value>,
}

pub struct NotificationRequest {
    method: String,
    params: Option<serde_json::Value>,
}

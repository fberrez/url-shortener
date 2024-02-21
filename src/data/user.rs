// use chrono::NaiveDateTime;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// enum Provider {
//     Google,
//     Github,
//     Apple,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct User {
//     pub user_id: i32,
//     pub username: String,
//     pub email: String,
//     pub created_at: NaiveDateTime,
//     pub provider: Provider,
//     pub google_id: Option<String>,
//     pub google_access_token: Option<String>,
//     pub google_refresh_token: Option<String>,
//     pub google_expires_in: Option<NaiveDateTime>,
//     pub github_id: Option<String>,
//     pub github_access_token: Option<String>,
//     pub github_refresh_token: Option<String>,
//     pub github_expires_in: Option<NaiveDateTime>,
//     pub apple_id: Option<String>,
//     pub apple_identity_token: Option<String>,
//     pub apple_access_token: Option<String>,
//     pub apple_refresh_token: Option<String>,
//     pub apple_expires_in: Option<NaiveDateTime>,
// }

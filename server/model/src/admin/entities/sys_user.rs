use derive_new::new;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, new)]
pub struct User {
    id: u32,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Role {
    Client {},
    Instructor {},
    Manager {},
    Admin {},
}

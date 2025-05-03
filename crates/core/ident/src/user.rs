use bson::oid::ObjectId;

#[derive(Debug, Clone)]
pub struct UserIdWithName {
    pub id: ObjectId,
    pub name: String,
}

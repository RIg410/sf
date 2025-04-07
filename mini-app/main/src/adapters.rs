use crate::pb::id::ObjectId as ObjectIdView;
use bson::oid::ObjectId;
use model::rights::HasRule;
use tonic::Status;

pub trait ToView<T> {
    fn to_view<R: HasRule>(self, rights: &R) -> T;
}

pub trait ToModel<T> {
    fn to_model(self) -> Result<T, Status>;
}

impl ToView<ObjectIdView> for ObjectId {
    fn to_view<R: HasRule>(self, _: &R) -> ObjectIdView {
        ObjectIdView {
            value: self.bytes().to_vec(),
        }
    }
}

impl ToModel<ObjectId> for ObjectIdView {
    fn to_model(self) -> Result<ObjectId, Status> {
        let mut bf = [0; 12];
        if self.value.len() != 12 {
            return Err(Status::invalid_argument("invalid id"));
        }
        bf.copy_from_slice(&self.value);
        Ok(ObjectId::from_bytes(bf))
    }
}

pub mod auth {
    tonic::include_proto!("auth");
}

pub mod user {
    tonic::include_proto!("user");
}

pub mod users {
    tonic::include_proto!("users");
}

pub mod locations {
    tonic::include_proto!("locations");
}

pub mod id {
    tonic::include_proto!("id");
}

pub mod subscription {
    tonic::include_proto!("subscription");
}

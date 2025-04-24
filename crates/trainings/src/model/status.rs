use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum TrainingStatus {
    OpenToSignup { close_sign_out: bool },
    ClosedToSignup,
    InProgress,
    Cancelled,
    Finished,
}

impl TrainingStatus {
    pub fn can_be_canceled(&self) -> bool {
        matches!(
            self,
            TrainingStatus::OpenToSignup { .. } | TrainingStatus::ClosedToSignup
        )
    }

    pub fn can_be_uncanceled(&self) -> bool {
        matches!(self, TrainingStatus::Cancelled)
    }

    pub fn can_sign_out(&self) -> bool {
        if let TrainingStatus::OpenToSignup { close_sign_out } = self {
            !close_sign_out
        } else {
            false
        }
    }

    pub fn can_sign_in(&self) -> bool {
        matches!(self, TrainingStatus::OpenToSignup { .. })
    }
}

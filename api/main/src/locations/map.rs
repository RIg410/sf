use crate::{
    adapters::{ToModel, ToView},
    pb::locations::{DayHoursView, HallView, LocationView, WorkingHoursView},
};
use chrono::{DateTime, Utc};
use locations::model::{DayHours, Hall, Location, WorkingHours};
use rights::HasRule;

impl ToView<LocationView> for Location {
    fn to_view<R: HasRule>(self, _: &R) -> LocationView {
        LocationView {
            id: Some(self.id.to_view(&())),
            name: self.name,
            address: self.address,
            working_hours: Some(self.working_hours.to_view(&())),
            halls: self.halls.into_iter().map(|h| h.to_view(&())).collect(),
            version: self.version,
        }
    }
}

impl ToView<WorkingHoursView> for WorkingHours {
    fn to_view<R: HasRule>(self, _: &R) -> WorkingHoursView {
        WorkingHoursView {
            monday: self.monday.map(|d| d.to_view(&())),
            tuesday: self.tuesday.map(|d| d.to_view(&())),
            wednesday: self.wednesday.map(|d| d.to_view(&())),
            thursday: self.thursday.map(|d| d.to_view(&())),
            friday: self.friday.map(|d| d.to_view(&())),
            saturday: self.saturday.map(|d| d.to_view(&())),
            sunday: self.sunday.map(|d| d.to_view(&())),
        }
    }
}

impl ToView<DayHoursView> for DayHours {
    fn to_view<R: HasRule>(self, _: &R) -> DayHoursView {
        DayHoursView {
            open: self.open.timestamp(),
            close: self.close.timestamp(),
        }
    }
}

impl ToView<HallView> for Hall {
    fn to_view<R: HasRule>(self, _: &R) -> HallView {
        HallView {
            id: Some(self.id.to_view(&())),
            name: self.name,
        }
    }
}

impl ToModel<WorkingHours> for WorkingHoursView {
    fn to_model(self) -> Result<WorkingHours, tonic::Status> {
        Ok(WorkingHours {
            monday: self.monday.map(|d| d.to_model()).transpose()?,
            tuesday: self.tuesday.map(|d| d.to_model()).transpose()?,
            wednesday: self.wednesday.map(|d| d.to_model()).transpose()?,
            thursday: self.thursday.map(|d| d.to_model()).transpose()?,
            friday: self.friday.map(|d| d.to_model()).transpose()?,
            saturday: self.saturday.map(|d| d.to_model()).transpose()?,
            sunday: self.sunday.map(|d| d.to_model()).transpose()?,
        })
    }
}

impl ToModel<DayHours> for DayHoursView {
    fn to_model(self) -> Result<DayHours, tonic::Status> {
        let open = DateTime::<Utc>::from_timestamp(self.open, 0)
            .ok_or_else(|| tonic::Status::invalid_argument("invalid open timestamp"))?;
        let close = DateTime::<Utc>::from_timestamp(self.close, 0)
            .ok_or_else(|| tonic::Status::invalid_argument("invalid close timestamp"))?;

        Ok(DayHours { open, close })
    }
}

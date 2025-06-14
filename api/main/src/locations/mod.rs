use crate::{
    adapters::{ToModel as _, ToView as _},
    ctx::ContextBuilder,
    pb::locations::{
        AddHallRequest, AddHallResponse, CreateLocationRequest, CreateLocationResponse,
        DeleteLocationRequest, DeleteLocationResponse, LocationListRequest, LocationListView,
        LocationRequest, LocationView, RemoveHallRequest, RemoveHallResponse, UpdateHallRequest,
        UpdateHallResponse, UpdateLocationRequest, UpdateLocationResponse,
        locations_service_server::LocationsService,
    },
};
use rights::Rule;
use tonic::async_trait;
use tracing::warn;

mod map;

#[derive(Clone)]
pub struct LocationsServer {
    context_builder: ContextBuilder,
}

impl LocationsServer {
    pub fn new(context_builder: ContextBuilder) -> Self {
        LocationsServer { context_builder }
    }
}

#[async_trait]
impl LocationsService for LocationsServer {
    async fn get(
        &self,
        request: tonic::Request<LocationRequest>,
    ) -> std::result::Result<tonic::Response<LocationView>, tonic::Status> {
        let mut ctx = self.context_builder.anonymous().await?;

        let location_id = request
            .into_inner()
            .id
            .map(|id| id.to_model())
            .transpose()?
            .ok_or_else(|| tonic::Status::invalid_argument("location id is required"))?;

        let location = ctx
            .services
            .locations
            .get(&mut ctx.session, location_id)
            .await
            .map_err(|err| {
                warn!("failed to get location: {}", err);
                tonic::Status::internal("failed to get location")
            })?
            .ok_or_else(|| tonic::Status::not_found("location not found"))?;

        Ok(tonic::Response::new(location.to_view(&())))
    }

    async fn list(
        &self,
        _: tonic::Request<LocationListRequest>,
    ) -> std::result::Result<tonic::Response<LocationListView>, tonic::Status> {
        let mut ctx = self.context_builder.anonymous().await?;

        let locations = ctx
            .services
            .locations
            .list(&mut ctx.session)
            .await
            .map_err(|err| {
                warn!("failed to list locations: {}", err);
                tonic::Status::internal("failed to list locations")
            })?;

        let locations = locations.into_iter().map(|l| l.to_view(&())).collect();

        Ok(tonic::Response::new(LocationListView { locations }))
    }

    async fn create(
        &self,
        request: tonic::Request<CreateLocationRequest>,
    ) -> std::result::Result<tonic::Response<CreateLocationResponse>, tonic::Status> {
        let mut ctx = self.context_builder.with_request(&request).await?;

        if !ctx.has_right(Rule::System) {
            return Err(tonic::Status::permission_denied("no rights"));
        }

        let req = request.into_inner();
        let working_hours = req
            .working_hours
            .ok_or_else(|| tonic::Status::invalid_argument("working_hours is required"))?
            .to_model()?;

        let id = ctx
            .services
            .locations
            .create(&mut ctx.session, req.name, req.address, working_hours)
            .await
            .map_err(|err| {
                warn!("failed to create location: {}", err);
                tonic::Status::internal("failed to create location")
            })?;

        Ok(tonic::Response::new(CreateLocationResponse {
            id: Some(id.to_view(&())),
        }))
    }

    async fn update(
        &self,
        request: tonic::Request<UpdateLocationRequest>,
    ) -> std::result::Result<tonic::Response<UpdateLocationResponse>, tonic::Status> {
        let mut ctx = self.context_builder.with_request(&request).await?;

        if !ctx.has_right(Rule::System) {
            return Err(tonic::Status::permission_denied("no rights"));
        }

        let req = request.into_inner();
        let location_id = req
            .id
            .ok_or_else(|| tonic::Status::invalid_argument("location id is required"))?
            .to_model()?;

        if let Some(name) = req.name {
            ctx.services
                .locations
                .update_location_name(&mut ctx.session, location_id, name)
                .await
                .map_err(|err| {
                    warn!("failed to update location name: {}", err);
                    tonic::Status::internal("failed to update location name")
                })?;
        }

        if let Some(address) = req.address {
            ctx.services
                .locations
                .update_location_address(&mut ctx.session, location_id, address)
                .await
                .map_err(|err| {
                    warn!("failed to update location address: {}", err);
                    tonic::Status::internal("failed to update location address")
                })?;
        }

        Ok(tonic::Response::new(UpdateLocationResponse {}))
    }

    async fn delete(
        &self,
        request: tonic::Request<DeleteLocationRequest>,
    ) -> std::result::Result<tonic::Response<DeleteLocationResponse>, tonic::Status> {
        let mut ctx = self.context_builder.with_request(&request).await?;

        if !ctx.has_right(Rule::System) {
            return Err(tonic::Status::permission_denied("no rights"));
        }

        let location_id = request
            .into_inner()
            .id
            .ok_or_else(|| tonic::Status::invalid_argument("location id is required"))?
            .to_model()?;

        ctx.services
            .locations
            .delete(&mut ctx.session, location_id)
            .await
            .map_err(|err| {
                warn!("failed to delete location: {}", err);
                tonic::Status::internal("failed to delete location")
            })?;

        Ok(tonic::Response::new(DeleteLocationResponse {}))
    }

    async fn add_hall(
        &self,
        request: tonic::Request<AddHallRequest>,
    ) -> std::result::Result<tonic::Response<AddHallResponse>, tonic::Status> {
        let mut ctx = self.context_builder.with_request(&request).await?;

        if !ctx.has_right(Rule::System) {
            return Err(tonic::Status::permission_denied("no rights"));
        }

        let req = request.into_inner();
        let location_id = req
            .location_id
            .ok_or_else(|| tonic::Status::invalid_argument("location id is required"))?
            .to_model()?;

        let hall_id = ctx
            .services
            .locations
            .add_hall(&mut ctx.session, location_id, req.hall_name)
            .await
            .map_err(|err| {
                warn!("failed to add hall: {}", err);
                tonic::Status::internal("failed to add hall")
            })?;

        Ok(tonic::Response::new(AddHallResponse {
            hall_id: Some(hall_id.to_view(&())),
        }))
    }

    async fn remove_hall(
        &self,
        request: tonic::Request<RemoveHallRequest>,
    ) -> std::result::Result<tonic::Response<RemoveHallResponse>, tonic::Status> {
        let mut ctx = self.context_builder.with_request(&request).await?;

        if !ctx.has_right(Rule::System) {
            return Err(tonic::Status::permission_denied("no rights"));
        }

        let req = request.into_inner();
        let location_id = req
            .location_id
            .ok_or_else(|| tonic::Status::invalid_argument("location id is required"))?
            .to_model()?;
        let hall_id = req
            .hall_id
            .ok_or_else(|| tonic::Status::invalid_argument("hall id is required"))?
            .to_model()?;

        ctx.services
            .locations
            .remove_hall(&mut ctx.session, location_id, hall_id)
            .await
            .map_err(|err| {
                warn!("failed to remove hall: {}", err);
                tonic::Status::internal("failed to remove hall")
            })?;

        Ok(tonic::Response::new(RemoveHallResponse {}))
    }

    async fn update_hall(
        &self,
        request: tonic::Request<UpdateHallRequest>,
    ) -> std::result::Result<tonic::Response<UpdateHallResponse>, tonic::Status> {
        let mut ctx = self.context_builder.with_request(&request).await?;

        if !ctx.has_right(Rule::System) {
            return Err(tonic::Status::permission_denied("no rights"));
        }

        let req = request.into_inner();
        let location_id = req
            .location_id
            .ok_or_else(|| tonic::Status::invalid_argument("location id is required"))?
            .to_model()?;
        let hall_id = req
            .hall_id
            .ok_or_else(|| tonic::Status::invalid_argument("hall id is required"))?
            .to_model()?;

        ctx.services
            .locations
            .update_hall_name(&mut ctx.session, location_id, hall_id, req.name)
            .await
            .map_err(|err| {
                warn!("failed to update hall name: {}", err);
                tonic::Status::internal("failed to update hall name")
            })?;

        Ok(tonic::Response::new(UpdateHallResponse {}))
    }
}

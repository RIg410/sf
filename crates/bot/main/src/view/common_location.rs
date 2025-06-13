use bot_core::{
    CommonLocation,
    context::Context,
    widget::{Jmp, ViewResult},
};
use bot_marketing::requests::Requests;
use bot_users::profile::UserProfile;
use rights::Rule;

pub async fn handle_common_location(ctx: &mut Context, location: CommonLocation) -> ViewResult {
    Ok(match location {
        CommonLocation::Profile(object_id) => {
            if ctx.has_right(Rule::ViewUsers) {
                UserProfile::new(object_id).into()
            } else {
                Jmp::Stay
            }
        }
        CommonLocation::Request(object_id) => {
            if ctx.has_right(Rule::ViewMarketingInfo) {
                Requests::new(None, true, Some(object_id)).into()
            } else {
                Jmp::Stay
            }
        }
    })
}

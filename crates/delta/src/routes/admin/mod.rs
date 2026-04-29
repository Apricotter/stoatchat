use revolt_rocket_okapi::revolt_okapi::openapi3::OpenApi;
use rocket::Route;

mod check_invitation;
mod create_invitation;
mod delete_invitation;
mod list_invitations;

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![
        create_invitation::create_invitation,
        list_invitations::list_invitations,
        delete_invitation::delete_invitation,
        check_invitation::check_invitation,
    ]
}

mod api;
mod nodeinfo;
mod users;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![api::statuses, nodeinfo::handler, users::statuses]
}

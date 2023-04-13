use crate::activitypub::object::Object;
use crate::dynamodb::DbSettings;

#[rocket::post("/api/v1/statuses", data = "<status>")]
pub async fn statuses(
    status: rocket::serde::json::Json<Object>,
    db_settings: &rocket::State<DbSettings>,
) -> Option<String> {
    let object = status.into_inner();
    let object_type = object.r#type.as_str();
    if object_type != "Note" {
        return None;
    }

    let partition = crate::dynamodb::get_uuid();
    crate::dynamodb::put_item(
        &db_settings.client,
        &db_settings.table_name,
        partition.as_str(),
        object,
    )
    .await
    .unwrap();
    Some(partition.to_string())
}

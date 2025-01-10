use std::collections::HashMap;
use actix_web::{post, get, put, delete, web, HttpResponse, Responder};
use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::types::AttributeValue;
use uuid::Uuid;
use crate::models::profile_request::{CreateProfileRequest, PutProfileRequest};

fn validate_visibility(visibility: &str) -> bool {
    matches!(visibility, "public" | "friends" | "private")
}

#[post("/profiles")]
async fn create_profile(profile: web::Json<CreateProfileRequest>, client: web::Data<Client>) -> impl Responder {
    let profile = profile.into_inner();

    if !validate_visibility(&profile.visibility) {
        return HttpResponse::BadRequest().body(format!("Invalid visibility: {}. \
            This should be public, friends, or private.", &profile.visibility));
    }

    match client
        .put_item()
        .table_name("Profiles")
        .item("user_id", AttributeValue::S(profile.user_id.clone()))
        .item("profile_id", AttributeValue::S(Uuid::new_v4().to_string()))
        .item("bio", AttributeValue::S(profile.bio.clone()))
        .item("visibility", AttributeValue::S(profile.visibility.clone()))
        .send()
        .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/profiles/{user_id}/{profile_id}")]
async fn get_profile(path: web::Path<(String, String)>, client: web::Data<Client>) -> impl Responder {
    let (user_id, profile_id) = path.into_inner();

    match client
        .get_item()
        .table_name("Profiles")
        .key("user_id", AttributeValue::S(user_id))
        .key("profile_id", AttributeValue::S(profile_id))
        .send()
        .await {
        Ok(output) => {
            if let Some(item) = output.item {
                let mut profile_out = HashMap::new();

                for (key, value) in item {
                    if let Ok(val) = value.as_s() {
                        profile_out.insert(key, val.clone());
                    }
                }

                HttpResponse::Ok().json(profile_out)
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/profiles/{user_id}/{profile_id}")]
async fn update_profile(
    path: web::Path<(String, String)>,
    updated_profile: web::Json<PutProfileRequest>,
    client: web::Data<Client>,
) -> impl Responder {
    let (user_id, profile_id) = path.into_inner();

    if !validate_visibility(&updated_profile.visibility) {
        return HttpResponse::BadRequest().body(format!("Invalid visibility: {}. \
            This should be public, friends, or private.", &updated_profile.visibility));
    }

    match client
        .get_item()
        .table_name("Profiles")
        .key("user_id", AttributeValue::S(user_id.clone()))
        .key("profile_id", AttributeValue::S(profile_id.clone()))
        .send()
        .await
    {
        Ok(response) => {
            if response.item.is_none() {
                return HttpResponse::NotFound().body("Profile not found. Cannot update a non-existent item.");
            }
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Failed to check item existence: {}", err));
        }
    }

    match client
        .update_item()
        .table_name("Profiles")
        .key("user_id", AttributeValue::S(user_id))
        .key("profile_id", AttributeValue::S(profile_id))
        .update_expression("SET #bio = :bio, #visibility = :visibility")
        .expression_attribute_names("#bio", "bio")
        .expression_attribute_names("#visibility", "visibility")
        .expression_attribute_values(":bio", AttributeValue::S(updated_profile.bio.clone()))
        .expression_attribute_values(":visibility", AttributeValue::S(updated_profile.visibility.clone()))
        .send()
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[delete("/profiles/{user_id}/all")]
async fn delete_all_profiles(user_id: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    match client
        .query()
        .table_name("Profiles")
        .key_condition_expression("user_id = :user_id")
        .expression_attribute_values(":user_id", AttributeValue::S(user_id.into_inner()))
        .send()
        .await
    {
        Ok(output) => {
            if let Some(items) = output.items {
                for item in items {
                    if let Some(user_id) = item.get("user_id") {
                        if let Some(profile_id) = item.get("profile_id") {
                            match client
                                .delete_item()
                                .table_name("Profiles")
                                .key("user_id", user_id.clone())
                                .key("profile_id", profile_id.clone())
                                .send()
                                .await
                            {
                                Ok(_) => (),
                                Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
                            }
                        }
                    }
                }
            }

            HttpResponse::Ok().finish()
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/profiles/{user_id}/{profile_id}")]
async fn delete_profile(path: web::Path<(String, String)>, client: web::Data<Client>) -> impl Responder {
    let (user_id, profile_id) = path.into_inner();

    match client
        .delete_item()
        .table_name("Profiles")
        .key("user_id", AttributeValue::S(user_id))
        .key("profile_id", AttributeValue::S(profile_id))
        .send()
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
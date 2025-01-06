use std::collections::HashMap;
use actix_web::{post, get, put, delete, web, HttpResponse, Responder};
use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::types::AttributeValue;
use uuid::Uuid;
use crate::models::profile::Profile;

#[post("/profiles")]
async fn create_profile(profile: web::Json<Profile>, client: web::Data<Client>) -> impl Responder {
    let profile = profile.into_inner();

    match client
        .put_item()
        .table_name("Profiles")
        .item("profile_id", AttributeValue::S(Uuid::new_v4().to_string()))
        .item("user_id", AttributeValue::S(profile.user_id.clone()))
        .item("bio", AttributeValue::S(profile.bio.clone()))
        .item("is_private", AttributeValue::Bool(profile.is_private.clone()))
        .send()
        .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/profiles/{id}")]
async fn get_profile(id: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    match client
        .get_item()
        .table_name("Profiles")
        .key("profile_id", AttributeValue::S(id.into_inner()))
        .send()
        .await
    {
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

#[put("/profiles/{id}")]
async fn update_profile(
    id: web::Path<String>,
    updated_item: web::Json<Profile>,
    client: web::Data<Client>,
) -> impl Responder {
    match client
        .update_item()
        .table_name("Profiles")
        .key("profile_id", AttributeValue::S(id.into_inner()))
        .update_expression("SET #bio = :bio, #is_private = :is_private")
        .expression_attribute_names("#bio", "bio")
        .expression_attribute_names("#is_private", "is_private")
        .expression_attribute_values(":bio", AttributeValue::S(updated_item.bio.clone()))
        .expression_attribute_values(":is_private", AttributeValue::Bool(updated_item.is_private.clone()))
        .send()
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/profiles/{id}")]
async fn delete_profile(id: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    match client
        .delete_item()
        .table_name("Profiles")
        .key("profile_id", AttributeValue::S(id.into_inner()))
        .send()
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
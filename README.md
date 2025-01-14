# profile_service

This is a microservice of exploredio which handles the user profiles in the application.
It's built with Rust, using the Actix Web framework, and stores data in Amazon DynamoDB.


The code below shows the fields of the current profile implementation.
Since users can have multiple profiles, each profile is linked to a unique user ID:
```rust
    user_id: String,
    profile_id: String,
    bio: String,
    visibility: String,
```
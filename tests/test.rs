use super*

#[test]
fn test_post_users() {
    let user = user_create("12345".to_string(), "Alice".to_string());
    assert_eq!(user.username, "Alice");
    assert_eq!(user.github_user_id, "12345");
    assert!(!user.id.is_empty())
}
fn test_get_users() {}

// treat specific user
fn test_get_user() {}
fn test_fetch_user() {}
fn test_delete_user() {}

// add new profile to db table
fn test_post_profiles() {}
fn test_get_profiles() {}

// treat specific user's profile
fn test_fetch_user_profile() {}
fn test_delete_user_profile() {}

// finaly add new column to connection relational db table?
fn test_post_connects() {}

fn test_get_user_connect() {}
fn test_fetch_user_connect() {}
fn test_delete_user_connect() {}

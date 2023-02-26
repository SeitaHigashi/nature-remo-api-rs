/*
 * Nature API
 *
 * Read/Write Nature Remo
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserAndRolesInner {
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::HomeInviteUser>>,
}

impl UserAndRolesInner {
    pub fn new() -> UserAndRolesInner {
        UserAndRolesInner {
            role: None,
            user: None,
        }
    }
}



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
pub struct LightState {
    #[serde(rename = "brightness", skip_serializing_if = "Option::is_none")]
    pub brightness: Option<String>,
    #[serde(rename = "last_button", skip_serializing_if = "Option::is_none")]
    pub last_button: Option<String>,
    #[serde(rename = "power", skip_serializing_if = "Option::is_none")]
    pub power: Option<String>,
}

impl LightState {
    pub fn new() -> LightState {
        LightState {
            brightness: None,
            last_button: None,
            power: None,
        }
    }
}



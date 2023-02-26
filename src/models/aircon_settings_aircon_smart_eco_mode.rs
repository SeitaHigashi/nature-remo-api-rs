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
pub struct AirconSettingsAirconSmartEcoMode {
    #[serde(rename = "adjusting", skip_serializing_if = "Option::is_none")]
    pub adjusting: Option<bool>,
    #[serde(rename = "area", skip_serializing_if = "Option::is_none")]
    pub area: Option<i32>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl AirconSettingsAirconSmartEcoMode {
    pub fn new() -> AirconSettingsAirconSmartEcoMode {
        AirconSettingsAirconSmartEcoMode {
            adjusting: None,
            area: None,
            enabled: None,
        }
    }
}



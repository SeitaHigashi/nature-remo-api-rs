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
pub struct ApplianceResponseSmartMeter {
    #[serde(rename = "echonetlite_properties", skip_serializing_if = "Option::is_none")]
    pub echonetlite_properties: Option<Vec<crate::models::ApplianceResponseSmartMeterEchonetlitePropertiesInner>>,
}

impl ApplianceResponseSmartMeter {
    pub fn new() -> ApplianceResponseSmartMeter {
        ApplianceResponseSmartMeter {
            echonetlite_properties: None,
        }
    }
}



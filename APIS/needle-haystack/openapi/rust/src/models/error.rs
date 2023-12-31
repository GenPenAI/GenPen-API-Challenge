/*
 * API Inspector
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "message")]
    pub message: String,
}

impl Error {
    pub fn new(code: i32, message: String) -> Error {
        Error {
            code,
            message,
        }
    }
}



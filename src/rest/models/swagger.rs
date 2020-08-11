use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwaggerApiDescription {
    pub swagger: String,
    pub info: SwaggerApiDescriptionInfo,
    pub base_path: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwaggerApiDescriptionInfo {
    pub title: String,
    pub description: String,
    pub terms_of_service: String,
    pub version: String,
}

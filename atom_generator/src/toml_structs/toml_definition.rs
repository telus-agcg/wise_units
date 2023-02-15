#[derive(Debug, Deserialize, Clone)]
pub(crate) struct TomlDefinition {
    #[serde(default = "default_value")]
    pub(crate) value: f64,

    #[serde(rename = "Unit")]
    pub(crate) unit: String,

    pub(crate) function: Option<TomlFunction>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct TomlFunction {
    //pub(crate) name: String,
    pub(crate) value: f64,
    #[serde(rename = "Unit")]
    pub(crate) unit: String,
}

const fn default_value() -> f64 {
    1.0
}

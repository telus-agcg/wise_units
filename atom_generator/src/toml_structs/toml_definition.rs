#[derive(Debug, Deserialize, Clone)]
pub(crate) struct TomlDefinition {
    #[serde(default = "default_value")]
    pub value: f64,

    #[serde(rename = "Unit")]
    pub unit: String,

    pub function: Option<TomlFunction>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct TomlFunction {
    pub name: String,
    pub value: f64,
    #[serde(rename = "Unit")]
    pub unit: String,
}

fn default_value() -> f64 {
    1.0
}

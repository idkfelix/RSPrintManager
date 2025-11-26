use super::types::CommandType;

#[derive(Debug, serde::Deserialize)]
pub struct Command {
  #[serde(rename = "type")]
  pub r#type: CommandType,
  pub value: Option<String>,
  pub printer: Option<String>,
}
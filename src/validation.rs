use std::collections::HashMap;

use anyhow::Result;
use regex::Regex;
use serde_json::Value;

pub fn validate_skill_name(skill_name: &str) -> Result<bool> {
    let re = Regex::new(r#"^[a-z0-9]([a-z0-9-]{0,62}[a-z0-9])?$"#)?;
    let is_match = re.is_match(skill_name);
    Ok(is_match)
}

pub fn validate_skill_description(skill_description: &str) -> bool {
    skill_description.len() <= 1024 && !skill_description.is_empty()
}

pub fn validate_skill_compatibility(compatibility: &str) -> bool {
    compatibility.len() <= 500 && !compatibility.is_empty()
}

pub fn validate_allowed_tools(allowed_tools: &str) -> bool {
    let tools: Vec<&str> = allowed_tools.split_whitespace().collect();
    !allowed_tools.is_empty() && !tools.is_empty()
}

pub fn validate_metadata(metadata: HashMap<String, Value>) -> bool {
    !metadata.is_empty()
}

pub fn validate_license(license: &str) -> bool {
    !license.is_empty()
}

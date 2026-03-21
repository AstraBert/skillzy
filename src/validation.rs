use anyhow::Result;
use regex::Regex;

pub fn validate_skill_name(skill_name: &str) -> Result<bool> {
    let re = Regex::new(r#"^[a-z0-9]([a-z0-9-]{0,62}[a-z0-9])?$"#)?;
    let is_match = re.is_match(skill_name);
    Ok(is_match)
}

pub fn validate_skill_description(skill_description: &str) -> bool {
    skill_description.len() <= 1024
}

pub fn validate_skill_compatibility(compatibility: &str) -> bool {
    compatibility.len() <= 500
}

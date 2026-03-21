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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_skill_name_valid_name() {
        let name = "pdf-reading";
        let is_valid = validate_skill_name(name).expect("Should be able to validate name");
        assert!(is_valid)
    }

    #[test]
    fn test_validate_skill_name_ends_with_hyphen() {
        let name = "pdf-reading-";
        let is_valid = validate_skill_name(name).expect("Should be able to validate name");
        assert!(!is_valid)
    }

    #[test]
    fn test_validate_skill_name_starts_with_hyphens() {
        let name = "-pdf-reading";
        let is_valid = validate_skill_name(name).expect("Should be able to validate name");
        assert!(!is_valid)
    }

    #[test]
    fn test_validate_skill_name_uppercase_chars() {
        let name = "PDF-reading";
        let is_valid = validate_skill_name(name).expect("Should be able to validate name");
        assert!(!is_valid)
    }

    #[test]
    fn test_validate_skill_description() {
        let desc = "useful skill";
        assert!(validate_skill_description(desc));
        let invalid = "1".repeat(1025);
        assert!(!validate_skill_description(&invalid));
        assert!(!validate_skill_description(""));
    }

    #[test]
    fn test_validate_skill_compatibility() {
        let desc = "is compatible";
        assert!(validate_skill_compatibility(desc));
        let invalid = "1".repeat(501);
        assert!(!validate_skill_compatibility(&invalid));
        assert!(!validate_skill_compatibility(""));
    }

    #[test]
    fn test_validate_allowed_tools() {
        let desc = "Bash(git:*)";
        assert!(validate_allowed_tools(desc));
        let desc1 = "Bash(git:*) Read";
        assert!(validate_allowed_tools(desc1));
        assert!(!validate_allowed_tools(""));
    }

    #[test]
    fn test_validate_metadata() {
        let mut meta: HashMap<String, Value> = HashMap::new();
        meta.insert("hello".to_string(), Value::from("world"));
        assert!(validate_metadata(meta));
        assert!(!validate_metadata(HashMap::new()))
    }

    #[test]
    fn test_validate_license() {
        let desc = "MIT";
        assert!(validate_license(desc));
        assert!(!validate_license(""));
    }
}

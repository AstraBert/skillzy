use std::{collections::HashMap, fs};

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::validation::{
    validate_allowed_tools, validate_license, validate_metadata, validate_skill_compatibility,
    validate_skill_description, validate_skill_name,
};

fn null_as_empty_map<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<HashMap<String, Value>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<HashMap<String, Value>>::deserialize(deserializer)?;
    match opt {
        Some(m) => Ok(Some(m)),
        None => Ok(Some(HashMap::new())),
    }
}

fn null_as_empty_string<'de, D>(deserializer: D) -> std::result::Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => Ok(Some(s)),
        None => Ok(Some(String::new())),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SkillFrontmatter {
    name: String,
    description: String,
    #[serde(default, deserialize_with = "null_as_empty_string")]
    compatibility: Option<String>,
    #[serde(
        default,
        rename = "allowed-tools",
        deserialize_with = "null_as_empty_string"
    )]
    allowed_tools: Option<String>,
    #[serde(default, deserialize_with = "null_as_empty_map")]
    metadata: Option<HashMap<String, Value>>,
    #[serde(default, deserialize_with = "null_as_empty_string")]
    license: Option<String>,
}

pub fn check(skill_file: &str) -> Result<()> {
    let content = fs::read_to_string(skill_file)?;
    let (frontmatter, _) = markdown_frontmatter::parse::<SkillFrontmatter>(&content)?;
    if !validate_skill_name(&frontmatter.name)? {
        return Err(anyhow!("`name` is not compliant with requirements"));
    }
    if !validate_skill_description(&frontmatter.description) {
        return Err(anyhow!(
            "`description` must be more than 0 and less than 1024 charachters in length"
        ));
    }
    if let Some(c) = frontmatter.compatibility
        && !validate_skill_compatibility(&c)
    {
        return Err(anyhow!(
            "`compatibility` must be more than 0 and less than 500 charachters in length"
        ));
    }
    if let Some(a) = frontmatter.allowed_tools
        && !validate_allowed_tools(&a)
    {
        return Err(anyhow!(
            "`allowed-tools` should be a string containing whitespace-separated tool names for the agent to use"
        ));
    }
    if let Some(m) = frontmatter.metadata
        && !validate_metadata(m)
    {
        return Err(anyhow!("`metadata` should have a non-zero length"));
    }

    if let Some(l) = frontmatter.license
        && !validate_license(&l)
    {
        return Err(anyhow!("`license` should contain something"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_valid() {
        let result = check("testfiles/valid.md");
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_invalid_name() {
        let result = check("testfiles/invalid-name.md");
        assert!(result.is_err_and(
            |e| e.to_string() == "`name` is not compliant with requirements".to_string()
        ));
    }

    #[test]
    fn test_check_invalid_desc() {
        let result = check("testfiles/invalid-desc.md");
        assert!(result.is_err_and(|e| {
            e.to_string()
                == "`description` must be more than 0 and less than 1024 charachters in length"
                    .to_string()
        }));
    }

    #[test]
    fn test_check_invalid_compat() {
        let result = check("testfiles/invalid-compat.md");
        assert!(result.is_err_and(|e| {
            e.to_string()
                == "`compatibility` must be more than 0 and less than 500 charachters in length"
                    .to_string()
        }));
    }

    #[test]
    fn test_check_invalid_license() {
        let result = check("testfiles/invalid-license.md");
        assert!(
            result
                .is_err_and(|e| e.to_string() == "`license` should contain something".to_string())
        );
    }

    #[test]
    fn test_check_invalid_tools() {
        let result = check("testfiles/invalid-tools.md");
        assert!(result.is_err_and(
            |e| e.to_string() == "`allowed-tools` should be a string containing whitespace-separated tool names for the agent to use".to_string()
        ));
    }

    #[test]
    fn test_check_invalid_meta() {
        let result = check("testfiles/invalid-meta.md");
        assert!(result.is_err_and(
            |e| e.to_string() == "`metadata` should have a non-zero length".to_string()
        ));
    }
}

use std::fs;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::validation::{
    validate_skill_compatibility, validate_skill_description, validate_skill_name,
};

#[derive(Debug, Serialize, Deserialize)]
struct SkillFrontmatter {
    name: String,
    description: String,
    #[serde(default)]
    compatibility: Option<String>,
}

pub fn check(skill_file: &str) -> Result<()> {
    let content = fs::read_to_string(skill_file)?;
    let (frontmatter, _) = markdown_frontmatter::parse::<SkillFrontmatter>(&content)?;
    if !validate_skill_name(&frontmatter.name)? {
        return Err(anyhow!("Skill name is not compliant with requirements"));
    }
    if !validate_skill_description(&frontmatter.description) {
        return Err(anyhow!(
            "Skill description must be less than 1024 charachters in length"
        ));
    }
    if let Some(c) = frontmatter.compatibility {
        if !validate_skill_compatibility(&c) {
            return Err(anyhow!(
                "Skill compatibility should be less than 500 charachters in length"
            ));
        }
    }

    Ok(())
}

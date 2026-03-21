use std::{fs, path::Path};

use anyhow::{Result, anyhow};

use crate::validation::{
    validate_allowed_tools, validate_skill_compatibility, validate_skill_description,
    validate_skill_name,
};

pub const SKILLS_DIR: &str = "skills/";
pub const SKILL_FILE: &str = "SKILL.md";

fn make_skill_directory(skill_name: &str) -> Result<()> {
    let path = Path::new(SKILLS_DIR);
    fs::create_dir_all(path.join(skill_name))?;
    Ok(())
}

fn transform_metadata(metadata: Vec<String>) -> Option<String> {
    if metadata.is_empty() {
        return None;
    }

    let mut metadata_str = "metadata:\n".to_string();
    for m in metadata {
        if let Some((key, value)) = m.split_once("=") {
            metadata_str += format!("  {}: {}\n", key, value).as_str();
        }
    }
    Some(metadata_str)
}

fn create_skill_frontmatter(
    skill_name: &str,
    skill_description: &str,
    compatibility: Option<&str>,
    license: Option<&str>,
    allowed_tools: Option<&str>,
    metadata: Vec<String>,
) -> Result<String> {
    if !validate_skill_name(skill_name)? {
        return Err(anyhow!("`name` is not compliant with requirements"));
    }
    if !validate_skill_description(skill_description) {
        return Err(anyhow!(
            "`description` must be more than 0 and less than 1024 charachters in length"
        ));
    }
    let mut frontmatter = format!(
        "---\nname: {}\ndescription: {}\n",
        skill_name,
        skill_description.replace("\n", " ")
    );
    if let Some(c) = compatibility {
        if !validate_skill_compatibility(c) {
            return Err(anyhow!(
                "`compatibility` must be more than 0 and less than 500 charachters in length"
            ));
        }
        frontmatter += format!("compatibility: {}\n", c).as_str();
    }
    if let Some(l) = license {
        frontmatter += format!("license: {}\n", l).as_str();
    }
    if let Some(a) = allowed_tools {
        if !validate_allowed_tools(a) {
            return Err(anyhow!(
                "`allowed-tools` should be a string containing whitespace-separated tool names for the agent to use"
            ));
        }
        frontmatter += format!("allowed-tools: {}\n", a).as_str();
    }
    if let Some(meta_string) = transform_metadata(metadata) {
        frontmatter += meta_string.as_str();
    }
    frontmatter += "---\n";
    Ok(frontmatter)
}

pub fn init_skill(
    skill_name: &str,
    skill_description: &str,
    compatibility: Option<&str>,
    license: Option<&str>,
    allowed_tools: Option<&str>,
    metadata: Vec<String>,
) -> Result<()> {
    let frontmatter = create_skill_frontmatter(
        skill_name,
        skill_description,
        compatibility,
        license,
        allowed_tools,
        metadata,
    )?;
    make_skill_directory(skill_name)?;
    let file_path = format!("{}{}/{}", SKILLS_DIR, skill_name, SKILL_FILE);
    fs::write(file_path, frontmatter)?;
    Ok(())
}

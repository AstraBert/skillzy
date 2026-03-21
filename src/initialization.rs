use std::{fs, path::Path};

use anyhow::{Result, anyhow};

use crate::validation::{
    validate_skill_compatibility, validate_skill_description, validate_skill_name,
};

pub const SKILLS_DIR: &str = "skills/";
pub const SKILL_FILE: &str = "SKILL.md";

fn make_skill_directory(skill_name: &str) -> Result<()> {
    let path = Path::new(SKILLS_DIR);
    fs::create_dir_all(path.join(skill_name))?;
    Ok(())
}

fn create_skill_frontmatter(
    skill_name: &str,
    skill_description: &str,
    compatibility: Option<&str>,
    license: Option<&str>,
) -> Result<String> {
    if !validate_skill_name(skill_name)? {
        return Err(anyhow!("Skill name is not compliant with requirements"));
    }
    if !validate_skill_description(skill_description) {
        return Err(anyhow!(
            "Skill description must be less than 1024 charachters in length"
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
                "Skill compatibility should be less than 500 charachters in length"
            ));
        }
        frontmatter += format!("compatibility: {}\n", c).as_str();
    }
    if let Some(l) = license {
        frontmatter += format!("license: {}\n", l).as_str();
    }
    frontmatter += "---\n";
    Ok(frontmatter)
}

pub fn init_skill(
    skill_name: &str,
    skill_description: &str,
    compatibility: Option<&str>,
    license: Option<&str>,
) -> Result<()> {
    let frontmatter =
        create_skill_frontmatter(skill_name, skill_description, compatibility, license)?;
    make_skill_directory(skill_name)?;
    let file_path = format!("{}{}/{}", SKILLS_DIR, skill_name, SKILL_FILE);
    fs::write(file_path, frontmatter)?;
    Ok(())
}

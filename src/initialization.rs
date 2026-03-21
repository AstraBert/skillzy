use std::{fs, path::Path};

use anyhow::{Result, anyhow};

use crate::validation::{
    validate_allowed_tools, validate_license, validate_skill_compatibility,
    validate_skill_description, validate_skill_name,
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
        if !validate_license(l) {
            return Err(anyhow!(
                "`license` must be more than 0 charachters in length"
            ));
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_metadata_some() {
        let meta: Vec<String> = vec!["key=value".to_string(), "hello=world".to_string()];
        let transformed = transform_metadata(meta);
        if let Some(t) = transformed {
            assert_eq!(t, "metadata:\n  key: value\n  hello: world\n");
        } else {
            assert!(false); // fail, it should return a non-None value
        }
    }

    #[test]
    fn test_transform_metadata_none() {
        let meta: Vec<String> = vec![];
        let transformed = transform_metadata(meta);
        assert!(transformed.is_none());
    }

    #[test]
    fn test_create_skill_frontmatter_all_options_none() {
        let frontmatter = create_skill_frontmatter("test", "Some skill", None, None, None, vec![])
            .expect("Should be able to create frontmatter");
        assert_eq!(
            frontmatter,
            "---\nname: test\ndescription: Some skill\n---\n"
        );
    }

    #[test]
    fn test_create_skill_frontmatter_compatibility() {
        let frontmatter = create_skill_frontmatter(
            "test",
            "Some skill",
            Some("is compatible"),
            None,
            None,
            vec![],
        )
        .expect("Should be able to create frontmatter");
        assert_eq!(
            frontmatter,
            "---\nname: test\ndescription: Some skill\ncompatibility: is compatible\n---\n"
        );
    }

    #[test]
    fn test_create_skill_frontmatter_license() {
        let frontmatter = create_skill_frontmatter(
            "test",
            "Some skill",
            Some("is compatible"),
            Some("MIT"),
            None,
            vec![],
        )
        .expect("Should be able to create frontmatter");
        assert_eq!(
            frontmatter,
            "---\nname: test\ndescription: Some skill\ncompatibility: is compatible\nlicense: MIT\n---\n"
        );
    }

    #[test]
    fn test_create_skill_frontmatter_allowed_tools() {
        let frontmatter = create_skill_frontmatter(
            "test",
            "Some skill",
            Some("is compatible"),
            Some("MIT"),
            Some("Bash(git:*) Read"),
            vec![],
        )
        .expect("Should be able to create frontmatter");
        assert_eq!(
            frontmatter,
            "---\nname: test\ndescription: Some skill\ncompatibility: is compatible\nlicense: MIT\nallowed-tools: Bash(git:*) Read\n---\n"
        );
    }

    #[test]
    fn test_create_skill_frontmatter_metadata() {
        let meta: Vec<String> = vec!["key=value".to_string(), "hello=world".to_string()];
        let frontmatter = create_skill_frontmatter(
            "test",
            "Some skill",
            Some("is compatible"),
            Some("MIT"),
            Some("Bash(git:*) Read"),
            meta,
        )
        .expect("Should be able to create frontmatter");
        assert_eq!(
            frontmatter,
            "---\nname: test\ndescription: Some skill\ncompatibility: is compatible\nlicense: MIT\nallowed-tools: Bash(git:*) Read\nmetadata:\n  key: value\n  hello: world\n---\n"
        );
    }

    #[test]
    fn test_create_skill_frontmatter_invalid_name() {
        let meta: Vec<String> = vec!["key=value".to_string(), "hello=world".to_string()];
        let frontmatter = create_skill_frontmatter(
            "test-",
            "Some skill",
            Some("is compatible"),
            Some("MIT"),
            Some("Bash(git:*) Read"),
            meta,
        );
        assert!(
            frontmatter
                .is_err_and(|e| { e.to_string() == "`name` is not compliant with requirements" })
        );
    }

    #[test]
    fn test_create_skill_frontmatter_invalid_desc() {
        let meta: Vec<String> = vec!["key=value".to_string(), "hello=world".to_string()];
        let frontmatter = create_skill_frontmatter(
            "test",
            "1".repeat(1025).as_str(),
            Some("is compatible"),
            Some("MIT"),
            Some("Bash(git:*) Read"),
            meta,
        );
        assert!(frontmatter.is_err_and(|e| {
            e.to_string()
                == "`description` must be more than 0 and less than 1024 charachters in length"
        }));
    }

    #[test]
    fn test_create_skill_frontmatter_invalid_compat() {
        let meta: Vec<String> = vec!["key=value".to_string(), "hello=world".to_string()];
        let frontmatter = create_skill_frontmatter(
            "test",
            "description",
            Some("1".repeat(501).as_str()),
            Some("MIT"),
            Some("Bash(git:*) Read"),
            meta,
        );
        assert!(frontmatter.is_err_and(|e| {
            e.to_string()
                == "`compatibility` must be more than 0 and less than 500 charachters in length"
        }));
    }

    #[test]
    fn test_create_skill_frontmatter_invalid_license() {
        let meta: Vec<String> = vec!["key=value".to_string(), "hello=world".to_string()];
        let frontmatter = create_skill_frontmatter(
            "test",
            "description",
            Some("is compatible"),
            Some(""),
            Some("Bash(git:*) Read"),
            meta,
        );
        assert!(frontmatter.is_err_and(|e| {
            e.to_string() == "`license` must be more than 0 charachters in length"
        }));
    }

    #[test]
    fn test_create_skill_frontmatter_invalid_allowed_tools() {
        let meta: Vec<String> = vec!["key=value".to_string(), "hello=world".to_string()];
        let frontmatter = create_skill_frontmatter(
            "test",
            "description",
            Some("is compatible"),
            Some("MIT"),
            Some(""),
            meta,
        );
        assert!(frontmatter.is_err_and(|e| {
            e.to_string() == "`allowed-tools` should be a string containing whitespace-separated tool names for the agent to use"
        }));
    }
}

mod check;
mod initialization;
mod validation;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    check::check,
    initialization::{SKILL_FILE, SKILLS_DIR, init_skill},
};

/// Initialize and check skills according to the specification adopted by agentskills.io and skills.sh
#[derive(Parser)]
#[command(version = "0.1.0")]
#[command(name = "skillzy")]
#[command(about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    cmd: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a skill by providing a name and a description, and optionally a compatibility description and a license.
    Init {
        /// Name of the skill. Max 64 charachters, lowercase letters, numbers, and hyphens only. Must not start or end with a hyphen.
        name: String,

        /// Description of the skill. Max 1024 charachters.
        #[arg(short, long)]
        description: String,

        /// Skill compatibility requirements. Max 500 charachters.
        #[arg(short, long, default_value=None)]
        compatibility: Option<String>,

        /// License name or reference to a bundled license file.
        #[arg(short, long, default_value=None)]
        license: Option<String>,
    },
    /// Check the frontmatter of a skill
    Check {
        /// Path to the skill file to check.
        path: String,
    },
}

fn main() -> Result<()> {
    let args = CliArgs::parse();
    match args.cmd {
        Commands::Init {
            name,
            description,
            compatibility,
            license,
        } => {
            init_skill(
                &name,
                &description,
                compatibility.as_deref(),
                license.as_deref(),
            )?;
            println!(
                "\x1b[1;37mSkill for \x1b[1;32m{}\x1b[1;37m successfully initialized at \x1b[1;35m{}{}/{}",
                name, SKILLS_DIR, name, SKILL_FILE
            );
        }
        Commands::Check { path } => {
            check(&path)?;
            println!(
                "\x1b[1;92mSuccess!\n\x1b[1;37mSkill at \x1b[1;35m{}\x1b[1;37m is compliant with the specification adopted by skills.sh and agentskills.io",
                path
            );
        }
    }
    Ok(())
}

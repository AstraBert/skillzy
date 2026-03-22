---
name: skillzy
description: Use this skill to initialize and check agent skills according to the agentskills.io and skills.sh specification. Trigger whenever the user wants to create a new skill file, validate existing skill frontmatter, or scaffold a skill with specific metadata (name, description, compatibility, license, allowed tools).
compatibility: Requires the skillzy CLI to be installed via Cargo (`cargo install skillzy`) or NPM (`npm install -g @cle-does-things/skillzy`).
license: MIT
metadata:
  author: Clelia Astra Bertelli
  version: 0.1.0
---

# `skillzy` Skill

Initialize and validate agent skill files according to the [agentskills.io](https://agentskills.io) and [skills.sh](https://skills.sh) specification.

## Initial Setup

When this skill is invoked, respond with:

```
I'm ready to use skillzy to initialize or validate skill files. Before we begin, please confirm that:

- `skillzy` is installed globally (`cargo install skillzy` or `npm install -g @cle-does-things/skillzy`)

If it's installed, please tell me what you'd like to do:

1. **Initialize** a new skill: provide a name and description (required), plus any optional fields: compatibility, license, allowed tools, or metadata key-value pairs
2. **Check** one or more existing skill files: provide the path(s) to the SKILL.md file(s) to validate

I will produce the appropriate `skillzy` command, and once execution is approved, report the results.
```

Then wait for the user's input.

---

## Step 0: Install skillzy (if needed)

If `skillzy` is not yet installed, install it globally:

- With **Cargo** (requires Rust):

```bash
cargo install skillzy
```

- With **NPM**:

```bash
npm install -g @cle-does-things/skillzy
```

Verify installation:

```bash
skillzy --help
```

---

## Step 1: Produce the CLI Command

### `init`: Initialize a New Skill File

```bash
# Minimal — name and description only
skillzy init my-skill --description "Does something useful"

# Full — with all optional fields
skillzy init my-skill \
  --description "Does something useful" \
  --compatibility "Requires Node.js 18+" \
  --license MIT \
  --allowed-tools "Bash(git:*)" \
  --metadata AUTHOR=alice \
  --metadata VERSION=1.0.0
```

### `check`: Validate Existing Skill File(s)

```bash
# Single file
skillzy check path/to/SKILL.md

# Multiple files
skillzy check path/to/SKILL.md path/to/another/SKILL.md
```

---

### Key Options Reference

**`init`**

| Flag | Alias | Description | Required |
|------|-------|-------------|----------|
| `NAME` | — | Skill name. Max 64 chars. Lowercase letters, numbers, hyphens only. Must not start/end with hyphen. | Yes |
| `--description` | `-d` | Skill description. Max 1024 characters. | Yes |
| `--compatibility` | `-c` | Compatibility requirements. Max 500 characters. | No |
| `--license` | `-l` | License name or path to bundled license file. | No |
| `--allowed-tools` | `-a` | Space-delimited list of pre-approved tools. Experimental. | No |
| `--metadata` | `-m` | Key-value pair `KEY=VALUE`. Repeatable. | No |

**`check`**

| Argument | Description | Required |
|----------|-------------|----------|
| `paths...` | One or more paths to SKILL.md files to validate. | Yes |

---

## Step 2: Execute and Report

Once the CLI command has been produced, ask for permission to execute and, if the permission is granted, run the command and report all results to the user.

--- 

If the user is working in a GitHub repository and wants to automate skills validation, you can set up a GHA workflow for that using the `AstraBert/run-skillzy` action:

```yaml
name: Check Skills Frontmatter

on:
  pull_request:

jobs:
  check-skills:
    name: Check Skills
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v6

      - name: Run Skillzy on all SKILL.md files in the skills/ directory
        uses: AstraBert/run-skillzy@v0.1.0
```

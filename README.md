# skillzy

**skillzy** is a CLI tool to initialize and validate skills according to the [agentskills.io](https://agentskills.io) and [skills.sh](https://skills.sh) specification.

## Installation

**Via Cargo:**
```bash
cargo install skillzy
```

**Via NPM:**
```bash
npm install -g @cle-does-things/skillzy
```

## Commands

### `init`

Initialize a new skill file by providing a name and description, with optional metadata.

```bash
skillzy init my-skill --description "Does something useful"
```

With all options:
```bash
skillzy init my-skill \
  --description "Does something useful" \
  --compatibility "Requires Node.js 18+" \
  --license MIT \
  --allowed-tools "Bash(git:*)" \
  --metadata AUTHOR=alice \
  --metadata VERSION=1.0.0
```

**Arguments:**
- `NAME` — Max 64 characters. Lowercase letters, numbers, and hyphens only. Must not start or end with a hyphen.

**Options:**
- `-d, --description` — Required. Max 1024 characters.
- `-c, --compatibility` — Optional. Skill compatibility requirements. Max 500 characters.
- `-l, --license` — Optional. License name or path to a bundled license file.
- `-a, --allowed-tools` — Optional. Space-delimited list of pre-approved tools the skill may use. Experimental.
- `-m, --metadata` — Optional. Key-value pair in the form `KEY=VALUE`. Can be repeated.

### `check`

Validate the frontmatter of one or more existing skill files.

```bash
skillzy check path/to/SKILL.md path/to/another/SKILL.md
```

## License

MIT

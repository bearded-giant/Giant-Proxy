# 🛡️ Giant Proxy

A Bash CLI wrapper around `mitmproxy` for managing regex-based request redirection using named profiles. Built for local testing of remote services. Supports both mitmproxy and Proxyman as backends.

## Features

- Named profiles stored in a single `rules.json` file
- Human-friendly rule IDs
- Enable/disable individual rules
- Verbose/minimal output modes
- Auto-generates mitmproxy Python scripts per profile
- Tracks last run profile (`last_run.json`)
- **NEW: HTTPS preservation for local development**
- **NEW: Structured JSON request/response logging**
- **NEW: Proxyman import/export compatibility**
- **NEW: Proxyman backend** for Node.js/SSR interception (see [PROXYMAN_USAGE.md](PROXYMAN_USAGE.md))
- Start and stop `mitmdump` cleanly in the background
- Bash autocompletion included

## What's New in v1.1.0

- **HTTPS Preservation**: Use `--preserve-https` to maintain secure connections when redirecting to local services
- **Structured Logging**: JSON-formatted request/response logs with filtering, timing, and correlation IDs
- **Proxyman Compatibility**: Import/export rules to easily migrate from Proxyman
- **Proxyman Backend**: Use Proxyman instead of mitmproxy for Node.js/SSR traffic interception
- **Enhanced CLI**: New commands for profile management, rule inspection, and git updates

## Setup

1. **Install `mitmproxy`**:

```bash
brew install mitmproxy
```

2. **Install the mitmproxy certificate** (basic flow):

```bash
mitmproxy  # generates the cert on first run
sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain ~/.mitmproxy/mitmproxy-ca-cert.pem
```

3. **Run the CLI from this folder (or install)**:

```bash
chmod +x giant-proxy
./giant-proxy install
```

### Add to your shell config if needed

```bash
export PATH="$HOME/.local/bin:$PATH"
```

## Commands

| Command                 | Description                                |
| ----------------------- | ------------------------------------------ |
| `list`, `ls`, `show`    | List rules in a profile                    |
| `toggle <id>`           | Toggle rule enabled/disabled               |
| `describe <id>`         | Show rule details                          |
| `add-rule <id> ...`     | Add a rule with profile, regex, host, port, scheme |
| `delete-rule <id>`      | Remove a rule from a profile               |
| `create-profile <name>` | Add a new profile                          |
| `start`, `on`           | Start proxy for profile (backgrounded)     |
| `stop`, `off`           | Stop running proxy                         |
| `status`                | Show if proxy is running                   |
| `logs`                  | View structured logs with filtering        |
| `config`                | Show/set backend configuration             |
| `doctor`                | Check dependencies for active backend      |
| `doctor --verbose`      | Detailed diagnostic output                 |
| `export`                | Output active rules in JSON                |
| `import-proxyman`       | Import rules from Proxyman                 |
| `export-proxyman`       | Export rules to Proxyman format            |
| `install`               | Symlink CLI to `~/.local/bin`              |
| `update`                | Pull latest version from git               |
| `which`                 | Show resolved CLI path                     |
| `version`               | Print version                              |
| `help`                  | Show command usage                         |

## Files

- `giant-proxy` — the CLI script
- `rules.json` — profiles + rules
- `logs/` — structured JSON logs
- `mitmproxy.log` — legacy log file
- `last_run.json` — info on last started profile
- `generated_proxy_map.py` — auto-generated mitmproxy script
- `~/.giant-proxy/` — config directory (backend settings, generated Proxyman configs)

## Examples

### Basic Usage
```bash
giant-proxy list --profile prod
giant-proxy toggle remix_merchant_portal_prod --profile prod
giant-proxy start --profile prod
giant-proxy logs
```

### HTTPS Preservation
```bash
# Maintain HTTPS when redirecting to local services
giant-proxy start --profile dev --preserve-https
```

### Advanced Logging
```bash
# Filter logs by keyword
giant-proxy logs --filter "POST"

# View logs as JSON
giant-proxy logs --json

# View logs without following (static)
giant-proxy logs --no-follow --filter "error"
```

### Proxyman Migration
```bash
# Import rules from Proxyman (supports Map Remote format)
giant-proxy import-proxyman proxyman_map_remote_rules.config --profile imported

# Export rules to Proxyman format (simple format)
giant-proxy export-proxyman --profile prod --output proxyman-export.json

# Export rules to Proxyman Map Remote format
giant-proxy export-proxyman --profile prod --output proxyman-map-remote.json --format map-remote
```

### Rule Management
```bash
# Add a rule with HTTPS scheme
giant-proxy add-rule api_rule --profile dev \
  --regex "^https://api\.example\.com/.*" \
  --host localhost \
  --port 3000 \
  --scheme https
```

### Proxyman Backend

For Node.js/SSR traffic that bypasses HTTP_PROXY, use Proxyman as the backend:

```bash
# Switch to Proxyman backend
giant-proxy config backend proxyman

# Start/stop works the same way
giant-proxy start --profile preprod
giant-proxy stop
```

See [PROXYMAN_USAGE.md](PROXYMAN_USAGE.md) for full setup and usage details.

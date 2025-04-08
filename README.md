# 🛡️ Giant Proxy

A Bash CLI wrapper around `mitmproxy` for managing regex-based request redirection using named profiles. Built for local testing of remote services.

## 📦 Features

- Named profiles stored in a single `rules.json` file
- Human-friendly rule IDs
- Enable/disable individual rules
- Verbose/minimal output modes
- Auto-generates mitmproxy Python scripts per profile
- Tracks last run profile (`last_run.json`)
- Streams logs to `mitmproxy.log`
- Start and stop `mitmdump` cleanly in the background
- Bash autocompletion included

## 🚀 Setup

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

## 🧰 Commands

| Command                 | Description                                |
| ----------------------- | ------------------------------------------ |
| `list`, `ls`, `show`    | List rules in a profile                    |
| `toggle <id>`           | Toggle rule enabled/disabled               |
| `describe <id>`         | Show rule details                          |
| `add-rule <id> ...`     | Add a rule with profile, regex, host, port |
| `delete-rule <id>`      | Remove a rule from a profile               |
| `create-profile <name>` | Add a new profile                          |
| `start`, `on`           | Start proxy for profile (backgrounded)     |
| `stop`, `off`           | Stop running proxy                         |
| `status`                | Show if mitmdump is running                |
| `logs`                  | Tail `mitmproxy.log`                       |
| `doctor`                | Check mitmdump/cert dependencies           |
| `doctor --verbose`      | Detailed diagnostic output                 |
| `export`                | Output active rules in JSON                |
| `install`               | Symlink CLI to `~/.local/bin`              |
| `which`                 | Show resolved CLI path                     |
| `version`               | Print version                              |
| `help`                  | Show command usage                         |

## 📄 Files

- `giant-proxy` — the CLI script
- `rules.json` — profiles + rules
- `mitmproxy.log` — background log file
- `last_run.json` — info on last started profile

## 📝 Example

```bash
giant-proxy list --profile prod
giant-proxy toggle remix_merchant_portal_prod --profile prod
giant-proxy start --profile prod
giant-proxy logs
```

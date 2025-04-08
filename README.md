# 🛡️ Giant Proxy

A wrapper Bash CLI wrapper around `mitmproxy` for managing regex-based request redirection with named profiles — created for local testing of remote services.

## 📦 Features

- Named profiles stored in a single `rules.json` file
- Human-friendly rule IDs instead of numeric indexes
- Enable/disable individual rules with `toggle`
- Minimal and verbose output modes
- Start/stop mitmproxy with tracking of last used profile
- Automatic generation of `mitmproxy` scripts
- Simple live logging with `tail -f mitmproxy.log`

## 🚀 Setup

1. Install [`mitmproxy`](https://mitmproxy.org)

   ```bash
   brew install mitmproxy
   ```

2. (Optional) Trust the mitmproxy root certificate:

   ```bash
   mitmproxy --install
   ```

3. Run the CLI from this folder:

   ```bash
   chmod +x giant-proxy
   ./giant-proxy list --profile prod
   ```

## 🚀 Installation (Recommended)

To install `giant-proxy` for use anywhere on your system:

```bash
cd giant-proxy
./giant-proxy install
```

This creates a symlink to `~/.local/bin/giant-proxy`, allowing you to run it from any terminal window like:

```bash
giant-proxy list --profile prod
```

If `~/.local/bin` is not in your `$PATH`, you may need to add this to your `~/.bashrc` or `~/.zshrc`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

## 🧰 CLI Commands

### 🔍 List rules

```bash
giant-proxy list --profile <profile> [--verbose]
```

### 🔁 Toggle a rule

```bash
giant-proxy toggle <rule_id> --profile <profile>
```

### 🚦 Start proxy with profile

```bash
giant-proxy start --profile <profile>
```

### 🛑 Stop proxy

```bash
giant-proxy stop
```

### 📊 Check status

```bash
giant-proxy status
```

### 📄 View logs

```bash
tail -f mitmproxy.log
```

Now `giant-proxy` works like a native command.

---

## 📁 Files

- `rules.json`: all your rules organized by profile
- `giant-proxy`: the CLI runner
- `generated_proxy_map.py`: auto-generated script for mitmproxy
- `last_run.json`: remembers your last used profile
- `mitmproxy.pid`: background process tracking
- `mitmproxy.log`: live logs

Now `giant-proxy` works like a native command.

---

# 🛡️ Giant Proxy

A feature-complete (pretty simple) Bash CLI wrapper around `mitmproxy` for managing regex-based request redirection with named profiles — created for local testing of remote services.

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

## 🌍 Making `giant-proxy` Global

To use `giant-proxy` from anywhere without `./`:

### Option 1: Add to PATH

1. Move or symlink the `giant-proxy` script into a directory in your `$PATH`.

   ```bash
   # Option A: Symlink
   ln -s /full/path/to/giant-proxy ~/bin/giant-proxy

   # Option B: Move it to a global bin folder
   sudo mv giant-proxy /usr/local/bin/
   ```

2. Make sure it's executable:

   ```bash
   chmod +x /usr/local/bin/giant-proxy
   ```

3. Now you can use it anywhere:

   ```bash
   giant-proxy list --profile prod
   ```

### Option 2: Shell Alias

If you prefer not to move the file, create an alias:

```bash
# Add this to your ~/.bashrc or ~/.zshrc
alias giant-proxy="/full/path/to/giant-proxy"
```

Then reload your shell:

```bash
source ~/.bashrc # or ~/.zshrc
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

## 🌍 Making `giant-proxy` Global

To use `giant-proxy` from anywhere without `./`:

### Option 1: Add to PATH

1. Move or symlink the `giant-proxy` script into a directory in your `$PATH`.

   ```bash
   # Option A: Symlink
   ln -s /full/path/to/giant-proxy ~/bin/giant-proxy

   # Option B: Move it to a global bin folder
   sudo mv giant-proxy /usr/local/bin/
   ```

2. Make sure it's executable:

   ```bash
   chmod +x /usr/local/bin/giant-proxy
   ```

3. Now you can use it anywhere:

   ```bash
   giant-proxy list --profile prod
   ```

### Option 2: Shell Alias

If you prefer not to move the file, create an alias:

```bash
# Add this to your ~/.bashrc or ~/.zshrc
alias giant-proxy="/full/path/to/giant-proxy"
```

Then reload your shell:

```bash
source ~/.bashrc # or ~/.zshrc
```

Now `giant-proxy` works like a native command.

---

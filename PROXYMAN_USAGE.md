# Giant-Proxy: Proxyman Backend Usage

Control Proxyman's Map Remote rules from the command line.

## How It Works

Giant-Proxy can use Proxyman as a backend instead of mitmproxy. This is useful when you need Node.js SSR traffic to be intercepted (mitmproxy only works for browser traffic since Node.js ignores `HTTP_PROXY`).

**Limitation**: The Proxyman CLI requires the app to be closed during import, so `start` and `stop` will quit and relaunch Proxyman automatically.

## Setup

```bash
# Switch to Proxyman backend
giant-proxy config backend proxyman

# Verify setup
giant-proxy doctor
```

## Import Your Proxyman Rules

Export your Map Remote rules from Proxyman (Tools > Map Remote > Export), then import:

```bash
# Import all rules to a single profile
giant-proxy import-proxyman ~/Downloads/proxyman-export.config --profile preprod

# Or auto-detect environments from URLs (prod/preprod/stage/prestage)
giant-proxy import-proxyman ~/Downloads/proxyman-export.config --auto-detect
```

## Daily Usage

```bash
# List available rules
giant-proxy list --profile preprod

# Start all enabled rules (closes and reopens Proxyman)
giant-proxy start --profile preprod

# Start a single rule only
giant-proxy start --profile preprod --rule remix_merchant_portal_preprod

# Check status
giant-proxy status

# Stop and close Proxyman
giant-proxy stop
```

## Toggle Rules

```bash
# Disable a rule (affects next start)
giant-proxy toggle remix_merchant_portal_preprod --profile preprod

# Re-enable it
giant-proxy toggle remix_merchant_portal_preprod --profile preprod
```

## Switch Back to mitmproxy

```bash
giant-proxy config backend mitmproxy
```

## Limitations

- Proxyman app is quit and restarted on `start` (CLI limitation - import only works when app is closed)
- Proxyman app is quit on `stop`
- Map Remote rules are appended on import

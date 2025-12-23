# Giant-Proxy: Proxyman Backend Usage

Control Proxyman's Map Remote rules from the command line.

## Setup

```bash
# Switch to Proxyman backend
giant-proxy config backend proxyman

# Verify setup
giant-proxy doctor
```

## Import Your Proxyman Rules

Export your Map Remote rules from Proxyman, then import:

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

# Start all enabled rules in profile
giant-proxy start --profile preprod

# Start a single rule only
giant-proxy start --profile preprod --rule preprod_remix_merchant_portal

# Check status
giant-proxy status

# Stop (clears imported rules from Proxyman)
giant-proxy stop
```

## Toggle Rules

```bash
# Disable a rule
giant-proxy toggle preprod_remix_merchant_portal --profile preprod

# Re-enable it
giant-proxy toggle preprod_remix_merchant_portal --profile preprod
```

## Switch Back to mitmproxy

```bash
giant-proxy config backend mitmproxy
```

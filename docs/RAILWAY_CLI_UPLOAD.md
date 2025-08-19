# Railway CLI Upload Guide

This document explains how to upload all prompt environment variables to Railway using the Railway CLI for advanced users.

## Prerequisites

1. **Install Railway CLI:**
   ```bash
   npm install -g @railway/cli
   ```

2. **Login and link your project:**
   ```bash
   railway login
   railway link  # Link your local project directory to Railway project
   ```

## Upload Methods

### Method 1: Automated Script (Recommended)

We provide a bash script that automatically uploads all `.env.*` files from the `prompt_envs/` directory:

```bash
# Dry run first to see what would be uploaded
./scripts/railway_upload_prompt_envs.sh --dry-run

# Actually upload the variables
./scripts/railway_upload_prompt_envs.sh
```

**Features:**
- ✅ Handles multiline values correctly
- ✅ Uses `--skip-deploys` to avoid triggering deployments
- ✅ Provides dry-run mode for safety
- ✅ Shows progress with emojis
- ✅ Handles all 10 prompt files automatically

### Method 2: Manual Upload (Alternative)

If you prefer manual control or the script doesn't work, you can upload individual files:

```bash
# Upload each file individually
railway variables set --file prompt_envs/.env.prompt_create_html --skip-deploys
railway variables set --file prompt_envs/.env.prompt_create_css --skip-deploys
railway variables set --file prompt_envs/.env.prompt_create_javascript --skip-deploys
# ... repeat for all files
```

### Method 3: Consolidated .env File

Create a single consolidated file and upload:

```bash
# Create consolidated file (optional)
cat prompt_envs/.env.* > .env.production

# Upload consolidated file
railway variables set --file .env.production --skip-deploys
```

## Verification

After upload, verify your variables are set correctly:

```bash
# List all variables (JSON format)
railway variables --json

# List all variables (table format)
railway variables

# Check specific variable
railway variables get prompt_create_html
```

## Current Prompt Variables

The system automatically loads these environment variables:

- `prompt_combined_research_validation` (12,799 chars)
- `prompt_create_css` (4,327 chars) 
- `prompt_create_html` (13,786 chars)
- `prompt_create_javascript` (17,086 chars)
- `prompt_create_report` (13,060 chars)
- `prompt_data_validation` (3,142 chars)
- `prompt_deep_research_report` (4,433 chars)
- `prompt_generate_report` (982 chars)
- `prompt_research_analysis` (1,709 chars)
- `prompt_translate_html` (1,244 chars)

## Troubleshooting

**Issue: "railway command not found"**
```bash
# Install Railway CLI globally
npm install -g @railway/cli
```

**Issue: "Project not linked"**
```bash
# Link your project
railway link
# Follow the prompts to select your Railway project
```

**Issue: "Variables not updating"**
```bash
# Trigger a deployment to reload environment variables
railway deploy
```

**Issue: "Multiline values truncated"**
- Use our automated script `./scripts/railway_upload_prompt_envs.sh`
- It handles multiline values correctly via temporary files

## Security Notes

- The `--skip-deploys` flag prevents automatic deployments during variable updates
- Environment variables are encrypted at rest on Railway
- Consider using Railway's secret management for sensitive data
- The upload script does not log variable values for security

## Support

- Railway CLI docs: https://docs.railway.app/reference/cli-api
- Railway environment variables: https://docs.railway.app/guides/environment-variables

#!/usr/bin/env bash
# Installs `wrangler`, the Cloudflare CLI used to deploy the static site to
# Cloudflare Pages (`wrangler pages deploy apps/site/dist`).
#
# Installed globally with npm, so this must run AFTER the Node install script.
# Authenticate separately with `wrangler login` or a CLOUDFLARE_API_TOKEN (plus
# CLOUDFLARE_ACCOUNT_ID) in the environment; credentials are never baked in.
set -euo pipefail

npm install -g wrangler

echo "Installed wrangler $(wrangler --version)"

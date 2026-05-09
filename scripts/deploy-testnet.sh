#!/usr/bin/env bash
# Deploy the energy-trade contract to Stellar testnet.
# Usage: ADMIN_SECRET_KEY=S... ./scripts/deploy-testnet.sh

set -euo pipefail

NETWORK="testnet"
WASM="contracts/target/wasm32v1-none/release/energy_trade.wasm"

if [ -z "${ADMIN_SECRET_KEY:-}" ]; then
  echo "Error: ADMIN_SECRET_KEY is not set." >&2
  exit 1
fi

echo "Building contract..."
(cd contracts && stellar contract build)

echo "Deploying to $NETWORK..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM" \
  --source-account "$ADMIN_SECRET_KEY" \
  --network "$NETWORK")

echo ""
echo "✅ Deployed successfully."
echo "CONTRACT_ID=$CONTRACT_ID"
echo ""
echo "Add this to backend/.env:"
echo "  CONTRACT_ID=$CONTRACT_ID"

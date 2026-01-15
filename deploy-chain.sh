#!/bin/bash
# ZOS Deployment Chain
# zos user -> systemd QA -> local prod -> Oracle -> Hugging Face
set -euo pipefail

ROOT="$HOME/meta-introspector"

echo "🚀 ZOS Deployment Chain"
echo "========================"
echo ""

# Step 1: Install as zos user (dev)
echo "📦 Step 1: Installing as zos user (dev)..."
sudo cp "$ROOT/deployments/local/zos/scripts/zos-zos.service" /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable zos-zos
sudo systemctl start zos-zos
sleep 2
sudo systemctl status zos-zos --no-pager || true
echo "✅ Dev service running as zos user"
echo ""

# Step 2: Deploy to systemd QA
echo "📦 Step 2: Deploying to systemd QA..."
sudo cp "$ROOT/deployments/local/qa-node/scripts/zos-qa-node.service" /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable zos-qa-node
sudo systemctl start zos-qa-node
sleep 2
sudo systemctl status zos-qa-node --no-pager || true
echo "✅ QA service running"
echo ""

# Step 3: Deploy to local prod
echo "📦 Step 3: Deploying to local prod..."
"$ROOT/tools/deploy.sh" deploy local prod-node prod
sudo cp "$ROOT/deployments/local/prod-node/scripts/zos-prod-node.service" /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable zos-prod-node
sudo systemctl start zos-prod-node
sleep 2
sudo systemctl status zos-prod-node --no-pager || true
echo "✅ Prod service running locally"
echo ""

# Step 4: Deploy to Oracle
echo "📦 Step 4: Deploying to Oracle Cloud..."
"$ROOT/tools/deploy.sh" deploy oracle node1 prod
echo "✅ Oracle deployment initiated"
echo ""

# Step 5: Deploy to Hugging Face
echo "📦 Step 5: Deploying to Hugging Face Spaces..."
"$ROOT/tools/deploy.sh" deploy huggingface space1 prod
echo "✅ Hugging Face deployment initiated"
echo ""

echo "🎉 Deployment chain complete!"
echo ""
echo "📊 Status:"
echo "  Dev (zos user):    sudo systemctl status zos-zos"
echo "  QA (systemd):      sudo systemctl status zos-qa-node"
echo "  Prod (local):      sudo systemctl status zos-prod-node"
echo "  Oracle:            Check Oracle Console"
echo "  Hugging Face:      Check HF Spaces"
echo ""
echo "📋 View all nodes:"
echo "  $ROOT/tools/deploy.sh list"

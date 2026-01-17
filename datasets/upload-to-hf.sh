#!/bin/bash
# Upload JSON dataset to HuggingFace

cd datasets/json-data

echo "Step 1: Login to HuggingFace"
echo "Run: huggingface-cli login"
echo "Or set token: export HF_TOKEN=your_token"
echo ""

# Create and upload
python3 << 'EOF'
from huggingface_hub import HfApi, login
import os

# Login (will use HF_TOKEN env var or cached token)
try:
    login()
    print("✓ Logged in to HuggingFace")
except:
    print("⚠ Please run: huggingface-cli login")
    exit(1)

# Create repo
api = HfApi()
try:
    api.create_repo(
        "h4ck3rm1k3/meta-introspector-json",
        repo_type="dataset",
        exist_ok=True
    )
    print("✓ Dataset repo created/verified")
except Exception as e:
    print(f"Error creating repo: {e}")
    exit(1)

# Upload files
try:
    api.upload_folder(
        folder_path=".",
        repo_id="h4ck3rm1k3/meta-introspector-json",
        repo_type="dataset",
    )
    print("✓ Uploaded 99 JSON files to HuggingFace")
    print("\nDataset available at:")
    print("https://huggingface.co/datasets/h4ck3rm1k3/meta-introspector-json")
except Exception as e:
    print(f"Error uploading: {e}")
    exit(1)
EOF

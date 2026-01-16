#!/usr/bin/env bash
# Start minimal build server
./target/debug/minimal-build-server &
echo "🚀 Server on http://127.0.0.1:3000"
echo "📦 Build: curl -X POST http://127.0.0.1:3000/build -H 'Content-Type: application/json' -d '{\"target\":\"git-sources\"}'"

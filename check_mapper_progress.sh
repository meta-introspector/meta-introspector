#!/bin/bash
echo "📊 Git File Mapper Progress"
echo "============================"
echo ""
tail -5 git_mapper_run3.log
echo ""
if pgrep -f "git_file_mapper" > /dev/null; then
    echo "✅ Status: Running"
    echo "⏱️  Check again in a few minutes"
else
    echo "✅ Status: Complete!"
    echo ""
    echo "Output files:"
    ls -lh FILE_GIT_MAPPING.csv data/indexes/files.parquet 2>/dev/null || echo "Files not found yet"
fi

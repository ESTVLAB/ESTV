#!/bin/bash

# ESTV Repository Stats Update Script
# Run this periodically to update repository statistics

set -e

echo "=========================================="
echo "ESTV Repository Stats Update"
echo "=========================================="
echo ""

# Get current date
DATE=$(date +%Y-%m-%d)

# Update last updated date in README (if needed)
# This can be customized based on your needs

echo "Updating repository statistics..."

# Count files
TOTAL_FILES=$(find . -type f -not -path './.git/*' | wc -l)
DOCS_COUNT=$(find ./docs -type f -name "*.md" | wc -l)
CODE_LINES=$(find ./programs -name "*.rs" -exec cat {} + 2>/dev/null | wc -l)

echo ""
echo "Repository Statistics:"
echo "----------------------"
echo "Total Files: $TOTAL_FILES"
echo "Documentation Files: $DOCS_COUNT"
echo "Lines of Rust Code: $CODE_LINES"
echo "Last Updated: $DATE"
echo ""

# Create or update stats file
cat > ./docs/STATS.md << EOF
# Repository Statistics

*Auto-generated on $DATE*

| Metric | Value |
|--------|-------|
| Total Files | $TOTAL_FILES |
| Documentation Files | $DOCS_COUNT |
| Lines of Code (Rust) | $CODE_LINES |
| Last Updated | $DATE |

## Commit Activity

\`\`\`
$(git log --oneline -10)
\`\`\`

## Contributors

$(git shortlog -sn --all | head -5)
EOF

echo "Stats file updated: ./docs/STATS.md"
echo ""
echo "=========================================="
echo "Update Complete"
echo "=========================================="

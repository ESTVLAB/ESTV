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

echo "Updating repository statistics..."

# Count files
TOTAL_FILES=$(find . -type f -not -path './.git/*' | wc -l)
DOCS_COUNT=$(find ./docs -type f -name "*.md" | wc -l)
CODE_LINES=$(find ./programs -name "*.rs" -exec cat {} + 2>/dev/null | wc -l)
TOTAL_COMMITS=$(git rev-list --count HEAD 2>/dev/null || echo "N/A")

echo ""
echo "Repository Statistics:"
echo "----------------------"
echo "Total Files: $TOTAL_FILES"
echo "Documentation Files: $DOCS_COUNT"
echo "Lines of Rust Code: $CODE_LINES"
echo "Total Commits: $TOTAL_COMMITS"
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
| Total Commits | $TOTAL_COMMITS |
| Exchange Listings | 1 (BitMart) |
| Last Updated | $DATE |

## Project Milestones

| Date | Milestone |
|------|-----------|
| Feb 23, 2026 | Initial repository setup |
| Feb 24, 2026 | CI/CD pipeline & documentation complete |
| Feb 25, 2026 | Treasury migration to Squads multisig |
| Feb 26, 2026 | BitMart ESTV/USDT listing live |

## Commit Activity (Recent)

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

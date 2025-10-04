#!/bin/bash
# Export all wiki markdown files to a single PDF
# Usage: ./scripts/export_wiki_to_pdf.sh

set -e

WIKI_DIR="$(dirname "$0")/../wiki"
OUTPUT_PDF="$(dirname "$0")/../chimera-wiki.pdf"
TMP_MD="/tmp/chimera_wiki_export.md"

# Check for required tools
if ! command -v pandoc >/dev/null 2>&1; then
  echo "Error: pandoc is required. Install with: brew install pandoc" >&2
  exit 1
fi

# Concatenate all markdown files in sorted order (Home first, then alphabetical)
echo "📚 Combining wiki pages..."
if [ -f "$WIKI_DIR/Home.md" ]; then
  cat "$WIKI_DIR/Home.md" > "$TMP_MD"
  echo -e "\n\n---\n\n" >> "$TMP_MD"
fi

# Add all other pages except Home
find "$WIKI_DIR" -name "*.md" ! -name "Home.md" -type f | sort | while read -r file; do
  cat "$file" >> "$TMP_MD"
  echo -e "\n\n---\n\n" >> "$TMP_MD"
done

echo "📄 Converting to PDF with pandoc..."

# Convert markdown directly to PDF using pandoc's built-in PDF engine
# This uses pdflatex/xelatex if available, or weasyprint as fallback
pandoc "$TMP_MD" \
  -o "$OUTPUT_PDF" \
  --pdf-engine=weasyprint \
  --metadata title="Chimera DSP Wiki" \
  --metadata author="Chimera Project" \
  --metadata date="$(date +%Y-%m-%d)" \
  --toc \
  --toc-depth=2 \
  --number-sections \
  -V geometry:margin=1in \
  -V linkcolor:blue \
  -V fontsize=11pt \
  2>/dev/null || {
    echo "⚠️  WeasyPrint not found. Trying with pdflatex..."
    pandoc "$TMP_MD" \
      -o "$OUTPUT_PDF" \
      --pdf-engine=pdflatex \
      --metadata title="Chimera DSP Wiki" \
      --metadata author="Chimera Project" \
      --metadata date="$(date +%Y-%m-%d)" \
      --toc \
      --toc-depth=2 \
      --number-sections \
      -V geometry:margin=1in \
      -V linkcolor:blue \
      -V fontsize=11pt
  }

rm -f "$TMP_MD"

if [ -f "$OUTPUT_PDF" ]; then
  SIZE=$(du -h "$OUTPUT_PDF" | cut -f1)
  echo "✅ Exported wiki to $OUTPUT_PDF ($SIZE)"
else
  echo "❌ PDF generation failed" >&2
  exit 1
fi

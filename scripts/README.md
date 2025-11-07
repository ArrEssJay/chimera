# Chimera Scripts

## `export_wiki_to_pdf.sh`

Exports all markdown files in the `wiki/` directory to a single PDF file (`chimera-wiki.pdf`).

### Usage

```bash
./scripts/export_wiki_to_pdf.sh
```

### Requirements
- `pandoc` (install with `brew install pandoc`)
- **Option 1** (Recommended): `weasyprint` (install with `pip3 install weasyprint`)
- **Option 2** (Fallback): Basic LaTeX distribution (comes with macOS or install with `brew install basictex`)

### Output
- PDF file: `chimera-wiki.pdf` in the project root
- Includes table of contents, page numbers, and formatting

### How it works
1. Concatenates all markdown files in `wiki/` (Home first, then alphabetical)
2. Converts directly to PDF using pandoc with weasyprint (or pdflatex as fallback)
3. Adds metadata (title, author, date), TOC, and section numbering

### Installation (Quick Start)

```bash
# Install pandoc
brew install pandoc

# Install weasyprint (Python-based PDF engine)
pip3 install weasyprint

# Or use LaTeX (larger download, ~4GB)
# brew install basictex
```

---

Feel free to add more scripts for automation!

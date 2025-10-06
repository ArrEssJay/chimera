# Wiki PDF Export Unicode Handling

## Problem
The wiki contains extensive Unicode characters that aren't natively supported by LaTeX (pdflatex with utf8 inputenc only supports U+0000-U+00FF). The `export_wiki_to_pdf.sh` script uses a Pandoc Lua filter to convert/strip Unicode.

## Solution Architecture
**File**: `scripts/unicode-to-latex.lua`

### Key Components

1. **Str() function** - Processes plain text nodes
   - Converts punctuation (em dash, curly quotes, ellipsis)
   - Converts Greek letters to `$\alpha$` etc.
   - Converts math operators to LaTeX math mode
   - Converts subscripts/superscripts using `\textsubscript{}` / `\textsuperscript{}`
   - Strips emoji and variation selectors (U+FE0F)
   - Logs and strips any remaining problematic Unicode

2. **CodeBlock() and Code() functions** - Processes code blocks and inline code
   - Box drawing characters → ASCII (`─` → `-`, `│` → `|`, `┼` → `+`)
   - Punctuation → ASCII equivalents
   - Strips remaining Unicode (simpler than Str since verbatim context)

### Character Classes Handled

**Punctuation** (U+2010-U+206F):
- U+2013 En dash (–) → `--`
- U+2014 Em dash (—) → `---`
- U+2018/2019 Curly quotes (') → `` ` `` / `'`
- U+201C/201D Curly quotes (") → ``` `` ``` / `''`
- U+2026 Ellipsis (…) → `\ldots{}`
- U+FE0F Variation Selector → stripped

**Box Drawing** (U+2500-U+257F):
- Horizontal → `-`
- Vertical → `|`
- Corners/junctions → `+`

**Math Operators** (U+2200-U+22FF):
- U+221A √ → `$\sqrt{}$`
- U+2212 − → `$-$`
- U+226A ≪ → `$\ll$`
- U+2225 ∥ → `$\parallel$`
- U+22A5 ⊥ → `$\perp$`
- Arrows → `$\leftarrow$` etc.

**Greek Letters** (U+0370-U+03FF):
- α, β, γ, δ, etc. → `$\alpha$`, `$\beta$`, etc.

**Special Symbols**:
- U+2112 ℒ (Laplacian) → `$\mathcal{L}$`
- U+2B50 ⭐ (star) → `$\star$`
- Time emojis (⏰⏱⏳) → `[clock]` `[timer]` `[hourglass]`

**Latin Extensions**:
- U+0124 Ĥ → `\^{H}`
- U+0131 ı (dotless i) → `\i{}`
- U+04E7 ӧ → `\"{o}`

**Subscripts/Superscripts**:
- ₀₁₂... → `\textsubscript{0}` etc.
- ⁰¹²³... → `\textsuperscript{0}` etc.

### Testing
```bash
# Clean log and test
rm -f /tmp/unicode_conversion.log
./scripts/export_wiki_to_pdf.sh

# Check what was stripped
cat /tmp/unicode_conversion.log | sort | uniq -c | sort -rn
```

### Common Issues

**Box drawing in code blocks** - Must use CodeBlock() handler, not just Str()
**Math in text** - Wrap in `$ $` for inline math mode
**Combining diacritics** - Strip these (U+0300-U+036F)
**Emoji** - Strip entirely, cannot represent in LaTeX

### UTF-8 Encoding Reference
- 1-byte: 0x00-0x7F (ASCII, handled natively)
- 2-byte: 0xC0-0xDF (U+0080-U+07FF)
- 3-byte: 0xE0-0xEF (U+0800-U+FFFF) ← Most problematic
- 4-byte: 0xF0-0xF4 (U+10000+, emojis)

Lua patterns use decimal: `\226` = 0xE2, `\148` = 0x94, etc.

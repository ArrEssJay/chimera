-- Comprehensive Pandoc Lua filter to convert Unicode to LaTeX-compatible format
--
-- LaTeX (pdflatex with utf8 inputenc) natively supports:
--   U+0000-U+007F: Basic Latin (ASCII)
--   U+0080-U+00FF: Latin-1 Supplement (most Western European accents)
--
-- Everything else must be converted or will cause errors.
-- This filter handles ALL problematic Unicode ranges comprehensively.
--
-- Uses UTF-8 byte patterns for reliable Unicode matching in Lua 5.3

function Str(elem)
  local text = elem.text
  
  -- ============================================================================
  -- STRIP: Emojis and Symbols (U+1F000-U+1FFFF, U+2600-U+27BF)
  -- ============================================================================
  -- Remove all emoji ranges completely (cannot be represented in LaTeX)
  text = text:gsub("[\240-\244][\128-\191]+", "")  -- UTF-8 emoji bytes
  text = text:gsub("\226[\152-\158][\128-\191]", "")  -- More symbols (U+2600-U+27BF)
  
  -- ============================================================================
  -- CONVERT: Box Drawing Characters (U+2500-U+257F)
  -- ============================================================================
  -- Box drawing is U+2500-U+257F
  -- In UTF-8: E2 94 80 to E2 95 BF
  -- Convert ALL box drawing characters to simple ASCII
  -- Range 1: U+2500-U+253F (E2 94 80 to E2 94 BF)
  text = text:gsub("\226\148[\128-\191]", function(match)
    local b3 = match:byte(3)
    -- Horizontal lines
    if b3 >= 0x80 and b3 <= 0x81 then return "-" end
    -- Vertical lines
    if b3 >= 0x82 and b3 <= 0x83 then return "|" end
    -- All corners and junctions
    return "+"
  end)
  
  -- Range 2: U+2540-U+257F (E2 95 80 to E2 95 BF)
  text = text:gsub("\226\149[\128-\191]", function(match)
    local b3 = match:byte(3)
    -- Horizontal lines
    if b3 >= 0x80 and b3 <= 0x81 then return "-" end
    -- Vertical lines  
    if b3 == 0x82 or b3 == 0x83 then return "|" end
    -- All corners and junctions
    return "+"
  end)
  
  -- ============================================================================
  -- CONVERT: Subscripts (U+2080-U+208E) using UTF-8 byte patterns
  -- ============================================================================
  text = text:gsub("\226\130\128", "\\textsubscript{0}")  -- ₀
  text = text:gsub("\226\130\129", "\\textsubscript{1}")  -- ₁
  text = text:gsub("\226\130\130", "\\textsubscript{2}")  -- ₂
  text = text:gsub("\226\130\131", "\\textsubscript{3}")  -- ₃
  text = text:gsub("\226\130\132", "\\textsubscript{4}")  -- ₄
  text = text:gsub("\226\130\133", "\\textsubscript{5}")  -- ₅
  text = text:gsub("\226\130\134", "\\textsubscript{6}")  -- ₆
  text = text:gsub("\226\130\135", "\\textsubscript{7}")  -- ₇
  text = text:gsub("\226\130\136", "\\textsubscript{8}")  -- ₈
  text = text:gsub("\226\130\137", "\\textsubscript{9}")  -- ₉
  text = text:gsub("\226\130\138", "\\textsubscript{+}")  -- ₊
  text = text:gsub("\226\130\139", "\\textsubscript{-}")  -- ₋
  text = text:gsub("\226\130\150", "\\textsubscript{k}")  -- ₖ U+2096
  text = text:gsub("\226\130\154", "\\textsubscript{p}")  -- ₚ U+209A
  
  -- ============================================================================
  -- CONVERT: Superscripts (U+2070-U+207F, plus ¹²³ from Latin-1)
  -- ============================================================================
  text = text:gsub("\226\129\176", "\\textsuperscript{0}")  -- ⁰
  text = text:gsub("\194\185", "\\textsuperscript{1}")      -- ¹
  text = text:gsub("\194\178", "\\textsuperscript{2}")      -- ²
  text = text:gsub("\194\179", "\\textsuperscript{3}")      -- ³
  text = text:gsub("\226\129\180", "\\textsuperscript{4}")  -- ⁴
  text = text:gsub("\226\129\181", "\\textsuperscript{5}")  -- ⁵
  text = text:gsub("\226\129\182", "\\textsuperscript{6}")  -- ⁶
  text = text:gsub("\226\129\183", "\\textsuperscript{7}")  -- ⁷
  text = text:gsub("\226\129\184", "\\textsuperscript{8}")  -- ⁸
  text = text:gsub("\226\129\185", "\\textsuperscript{9}")  -- ⁹
  text = text:gsub("\226\129\186", "\\textsuperscript{+}")  -- ⁺
  text = text:gsub("\226\129\187", "\\textsuperscript{-}")  -- ⁻
  text = text:gsub("\226\129\191", "\\textsuperscript{n}")  -- ⁿ
  text = text:gsub("\226\129\189", "(")  -- ⁽ U+207D - just use regular paren
  text = text:gsub("\226\129\190", ")")  -- ⁾ U+207E - just use regular paren
  
  -- ============================================================================
  -- CONVERT: Greek Letters (U+0370-U+03FF) using UTF-8 byte patterns
  -- ============================================================================
  -- Lowercase
  text = text:gsub("\206\177", "$\\alpha$")     -- α
  text = text:gsub("\206\178", "$\\beta$")      -- β
  text = text:gsub("\206\179", "$\\gamma$")     -- γ
  text = text:gsub("\206\180", "$\\delta$")     -- δ
  text = text:gsub("\206\181", "$\\epsilon$")   -- ε
  text = text:gsub("\206\182", "$\\zeta$")      -- ζ
  text = text:gsub("\206\183", "$\\eta$")       -- η
  text = text:gsub("\206\184", "$\\theta$")     -- θ
  text = text:gsub("\206\185", "$\\iota$")      -- ι
  text = text:gsub("\206\186", "$\\kappa$")     -- κ
  text = text:gsub("\206\187", "$\\lambda$")    -- λ
  text = text:gsub("\206\188", "$\\mu$")        -- μ
  text = text:gsub("\206\189", "$\\nu$")        -- ν
  text = text:gsub("\206\190", "$\\xi$")        -- ξ
  text = text:gsub("\207\128", "$\\pi$")        -- π
  text = text:gsub("\207\129", "$\\rho$")       -- ρ
  text = text:gsub("\207\131", "$\\sigma$")     -- σ
  text = text:gsub("\207\132", "$\\tau$")       -- τ
  text = text:gsub("\207\133", "$\\upsilon$")   -- υ
  text = text:gsub("\207\134", "$\\phi$")       -- φ
  text = text:gsub("\207\135", "$\\chi$")       -- χ
  text = text:gsub("\207\136", "$\\psi$")       -- ψ
  text = text:gsub("\207\137", "$\\omega$")     -- ω
  -- Uppercase  
  text = text:gsub("\206\147", "$\\Gamma$")     -- Γ
  text = text:gsub("\206\148", "$\\Delta$")     -- Δ
  text = text:gsub("\206\152", "$\\Theta$")     -- Θ
  text = text:gsub("\206\155", "$\\Lambda$")    -- Λ
  text = text:gsub("\206\158", "$\\Xi$")        -- Ξ
  text = text:gsub("\206\160", "$\\Pi$")        -- Π
  text = text:gsub("\206\163", "$\\Sigma$")     -- Σ
  text = text:gsub("\206\166", "$\\Phi$")       -- Φ
  text = text:gsub("\206\168", "$\\Psi$")       -- Ψ
  text = text:gsub("\206\169", "$\\Omega$")     -- Ω
  
  -- ============================================================================
  -- CONVERT: Mathematical Operators (U+2200-U+22FF) using UTF-8 byte patterns
  -- ============================================================================
  -- Comparison
  text = text:gsub("\226\137\164", "$\\leq$")         -- ≤
  text = text:gsub("\226\137\165", "$\\geq$")         -- ≥
  text = text:gsub("\226\137\160", "$\\neq$")         -- ≠
  text = text:gsub("\226\137\136", "$\\approx$")      -- ≈
  -- Arithmetic
  text = text:gsub("\195\151", "$\\times$")           -- ×
  text = text:gsub("\194\183", "$\\cdot$")            -- ·
  text = text:gsub("\195\183", "$\\div$")             -- ÷
  text = text:gsub("\194\177", "$\\pm$")              -- ±
  -- Set theory
  text = text:gsub("\226\136\136", "$\\in$")          -- ∈
  text = text:gsub("\226\136\137", "$\\ni$")          -- ∋
  text = text:gsub("\226\138\130", "$\\subset$")      -- ⊂
  text = text:gsub("\226\138\131", "$\\supset$")      -- ⊃
  text = text:gsub("\226\138\134", "$\\subseteq$")    -- ⊆
  text = text:gsub("\226\138\135", "$\\supseteq$")    -- ⊇
  text = text:gsub("\226\136\170", "$\\cup$")         -- ∪
  text = text:gsub("\226\136\169", "$\\cap$")         -- ∩
  text = text:gsub("\226\136\133", "$\\emptyset$")    -- ∅
  -- Logic
  text = text:gsub("\226\136\128", "$\\forall$")      -- ∀
  text = text:gsub("\226\136\131", "$\\exists$")      -- ∃
  text = text:gsub("\194\172", "$\\neg$")             -- ¬
  text = text:gsub("\226\136\167", "$\\wedge$")       -- ∧
  text = text:gsub("\226\136\168", "$\\vee$")         -- ∨
  -- Calculus
  text = text:gsub("\226\136\171", "$\\int$")         -- ∫
  text = text:gsub("\226\136\145", "$\\sum$")         -- ∑
  text = text:gsub("\226\136\143", "$\\prod$")        -- ∏
  text = text:gsub("\226\136\130", "$\\partial$")     -- ∂
  text = text:gsub("\226\136\135", "$\\nabla$")       -- ∇
  text = text:gsub("\226\136\158", "$\\infty$")       -- ∞
  -- Relations
  text = text:gsub("\226\136\157", "$\\propto$")      -- ∝
  
  -- ============================================================================
  -- CONVERT: Arrows (U+2190-U+21FF) using UTF-8 byte patterns
  -- ============================================================================
  text = text:gsub("\226\134\144", "$\\leftarrow$")      -- ← U+2190
  text = text:gsub("\226\134\145", "$\\uparrow$")        -- ↑ U+2191
  text = text:gsub("\226\134\146", "$\\rightarrow$")     -- → U+2192
  text = text:gsub("\226\134\147", "$\\downarrow$")      -- ↓ U+2193
  text = text:gsub("\226\134\148", "$\\leftrightarrow$") -- ↔ U+2194
  text = text:gsub("\226\134\149", "$\\updownarrow$")    -- ↕ U+2195
  text = text:gsub("\226\134\150", "$\\nwarrow$")        -- ↖ U+2196
  text = text:gsub("\226\134\151", "$\\nearrow$")        -- ↗ U+2197
  text = text:gsub("\226\134\152", "$\\searrow$")        -- ↘ U+2198
  text = text:gsub("\226\134\153", "$\\swarrow$")        -- ↙ U+2199
  text = text:gsub("\226\135\144", "$\\Leftarrow$")      -- ⇐ U+21D0
  text = text:gsub("\226\135\146", "$\\Rightarrow$")     -- ⇒ U+21D2
  text = text:gsub("\226\135\148", "$\\Leftrightarrow$") -- ⇔ U+21D4
  
  -- ============================================================================
  -- CONVERT: Punctuation and Typography (U+2010-U+206F)
  -- ============================================================================
  -- Dashes
  text = text:gsub("\226\128\144", "-")              -- U+2010 Hyphen
  text = text:gsub("\226\128\147", "--")             -- U+2013 En dash (–)
  text = text:gsub("\226\128\148", "---")            -- U+2014 Em dash (—)
  -- Quotes
  text = text:gsub("\226\128\152", "`")              -- U+2018 Left single quote (')
  text = text:gsub("\226\128\153", "'")              -- U+2019 Right single quote/apostrophe (')
  text = text:gsub("\226\128\156", "``")             -- U+201C Left double quote (")
  text = text:gsub("\226\128\157", "''")             -- U+201D Right double quote (")
  -- Ellipsis
  text = text:gsub("\226\128\166", "\\ldots{}")      -- U+2026 Horizontal ellipsis (…)
  -- Variation selectors (emoji modifiers) - just strip these
  text = text:gsub("\239\184\143", "")               -- U+FE0F Variation Selector-16
  
  -- ============================================================================
  -- CONVERT: Additional Mathematical Operators
  -- ============================================================================
  text = text:gsub("\226\136\154", "$\\sqrt{}$")     -- U+221A Square root (√)
  text = text:gsub("\226\138\165", "$\\perp$")       -- U+22A5 Up tack/perpendicular (⊥)
  text = text:gsub("\226\136\165", "$\\parallel$")   -- U+2225 Parallel to (∥)
  text = text:gsub("\226\137\170", "$\\ll$")         -- U+226A Much less than (≪)
  text = text:gsub("\226\137\171", "$\\gg$")         -- U+226B Much greater than (≫)
  text = text:gsub("\226\136\146", "$-$")            -- U+2212 Minus sign (−)
  
  -- ============================================================================
  -- CONVERT: Miscellaneous Symbols (U+2000-U+206F, U+2100-U+214F)
  -- ============================================================================
  local misc = {
    -- Bullets and shapes
    ["•"] = "$\\bullet$", ["◦"] = "$\\circ$", ["∘"] = "$\\circ$",
    ["★"] = "$\\star$", ["☆"] = "$\\star$", ["⭐"] = "$\\star$",
    ["□"] = "$\\square$", ["■"] = "$\\blacksquare$",
    ["○"] = "$\\bigcirc$", ["●"] = "$\\bullet$",
    ["△"] = "$\\triangle$", ["▽"] = "$\\nabla$",
    -- Degree and special
    ["°"] = "$^\\circ$", ["′"] = "$'$", ["″"] = "$''$",
    ["ℓ"] = "$\\ell$", ["ℏ"] = "$\\hbar$", ["℘"] = "$\\wp$",
    ["ℜ"] = "$\\Re$", ["ℑ"] = "$\\Im$", ["ℵ"] = "$\\aleph$",
    ["ℒ"] = "$\\mathcal{L}$",  -- U+2112 Script capital L (Laplacian)
    -- Time symbols
    ["⏰"] = "[clock]", ["⏱"] = "[timer]", ["⏳"] = "[hourglass]",
    -- Fractions (from Latin-1 Supplement)
    ["¼"] = "$\\frac{1}{4}$", ["½"] = "$\\frac{1}{2}$",
    ["¾"] = "$\\frac{3}{4}$"
  }
  for unicode, latex in pairs(misc) do
    text = text:gsub(unicode, latex)
  end
  
  -- ============================================================================
  -- CONVERT: Special Latin Extensions (U+0100-U+04FF)
  -- ============================================================================
  -- Latin Extended-A (Ĥ, ı, ӧ etc.)
  text = text:gsub("\196\164", "\\^{H}")             -- U+0124 Latin capital H with circumflex (Ĥ)
  text = text:gsub("\196\177", "\\i{}")              -- U+0131 Latin small letter dotless i (ı)
  text = text:gsub("\211\167", "\\\"{o}")            -- U+04E7 Cyrillic small letter o with diaeresis (ӧ)
  -- Combining diacritics - strip these as they're usually formatting artifacts
  text = text:gsub("\204[\128-\191]", "")            -- U+0300-U+033F Combining diacriticals
  text = text:gsub("\205[\128-\191]", "")            -- U+0340-U+036F More combining marks
  
  -- ============================================================================
  -- CONVERT: Modifier Letters and Phonetic Extensions (U+02B0-U+02FF, U+A700-U+A71F)
  -- ============================================================================
  text = text:gsub("\201[\144-\191]", "")            -- Strip IPA extensions
  text = text:gsub("\169\128\128", "")               -- U+A700 modifier letter chi (꜀)
  
  -- ============================================================================
  -- LOG and STRIP: Any remaining problematic Unicode (U+0100-U+FFFF)
  -- ============================================================================
  -- Convert remaining UTF-8 multibyte sequences using byte patterns
  -- UTF-8 encoding: 2-byte (0xC0-0xDF), 3-byte (0xE0-0xEF), 4-byte (0xF0-0xF4)
  
  -- Log file for debugging
  local log_file = "/tmp/unicode_conversion.log"
  local log = io.open(log_file, "a")
  
  -- Strip all 3-byte UTF-8 sequences (U+0800-U+FFFF) not already handled
  text = text:gsub("[\226-\239][\128-\191][\128-\191]", function(match)
    if log then
      -- Get UTF-8 bytes
      local b1, b2, b3 = match:byte(1, 3)
      local codepoint = ((b1 - 0xE0) * 0x1000) + ((b2 - 0x80) * 0x40) + (b3 - 0x80)
      log:write(string.format("Stripped U+%04X (%s) in text context\n", codepoint, match))
    end
    return ""  -- Strip the character
  end)
  
  -- Strip all 2-byte UTF-8 sequences (U+0100-U+07FF) not in Latin-1
  text = text:gsub("[\196-\223][\128-\191]", function(match)
    if log then
      local b1, b2 = match:byte(1, 2)
      local codepoint = ((b1 - 0xC0) * 0x40) + (b2 - 0x80)
      if codepoint >= 0x0100 then  -- Above Latin-1 Supplement
        log:write(string.format("Stripped U+%04X (%s) in text context\n", codepoint, match))
      end
    end
    return ""
  end)
  
  if log then
    log:close()
  end
  
  elem.text = text
  return elem
end

-- Function to process code blocks and inline code
-- We need a simpler conversion here since these are verbatim contexts
function process_code_text(text)
  -- Convert box drawing characters to ASCII in code blocks
  text = text:gsub("\226\148[\128-\191]", function(match)
    local b3 = match:byte(3)
    if b3 >= 0x80 and b3 <= 0x81 then return "-" end  -- Horizontal
    if b3 >= 0x82 and b3 <= 0x83 then return "|" end  -- Vertical
    return "+"  -- Corners and junctions
  end)
  text = text:gsub("\226\149[\128-\191]", function(match)
    local b3 = match:byte(3)
    if b3 >= 0x80 and b3 <= 0x81 then return "-" end  -- Horizontal
    if b3 == 0x82 or b3 == 0x83 then return "|" end   -- Vertical
    return "+"  -- Corners and junctions
  end)
  
  -- Convert common punctuation
  text = text:gsub("\226\128\148", "---")         -- Em dash
  text = text:gsub("\226\128\147", "--")          -- En dash
  text = text:gsub("\226\128\153", "'")           -- Right single quote
  text = text:gsub("\226\128\152", "`")           -- Left single quote
  text = text:gsub("\226\128\166", "...")         -- Ellipsis
  
  -- Strip emoji modifiers
  text = text:gsub("\239\184\143", "")            -- U+FE0F
  
  -- Strip any remaining problematic Unicode
  text = text:gsub("[\226-\239][\128-\191][\128-\191]", "")
  text = text:gsub("[\196-\223][\128-\191]", "")
  
  return text
end

function CodeBlock(elem)
  elem.text = process_code_text(elem.text)
  return elem
end

function Code(elem)
  elem.text = process_code_text(elem.text)
  return elem
end

# Wiki TODO - Future Pages

This page tracks planned wiki pages that are currently placeholders. These pages are referenced throughout the wiki but not yet created.

---

## 📋 Overview

**Total Placeholders**: 16 pages  
**Status**: Prioritized by category and referenced frequency

---

## 🎯 High Priority Pages

### Practical Guides
These pages are referenced in core learning paths and would help users apply theory:

- **[[Reading the Constellation]]** - How to interpret constellation diagrams
  - Referenced in: Constellation-Diagrams.md, Signal-Chain-(End-to-End-Processing).md
  - Purpose: Practical guide for users learning to debug signals
  - Content: Identify noise, phase errors, amplitude imbalance, IQ gain mismatch

- **[[Understanding BER Curves]]** - How to read and interpret bit error rate plots
  - Referenced in: Bit-Error-Rate-(BER).md, Forward-Error-Correction-(FEC).md
  - Purpose: Help users understand performance metrics
  - Content: Waterfall curves, Eb/N0 interpretation, comparing modulation schemes

---

## 📡 Modulation & Multiple Access

### Basic Modulation Shortcuts
These are shorthand references to existing pages - can be redirected:

- **[[BPSK]]** - Redirect to [[Binary-Phase-Shift-Keying-(BPSK)]]
  - Referenced in: Frequency-Shift-Keying-(FSK).md, On-Off-Keying-(OOK).md
  - Action: Update references to use full page name

- **[[FSK]]** - Redirect to [[Frequency-Shift-Keying-(FSK)]]
  - Referenced in: Free-Space-Path-Loss-(FSPL).md
  - Action: Update references to use full page name

- **[[16-QAM]]** - Should link to [[Quadrature-Amplitude-Modulation-(QAM)]]
  - Referenced in: On-Off-Keying-(OOK).md
  - Action: Update reference to QAM page, add 16-QAM section

### Advanced Topics

- **[[CDMA (coming soon)]]** - Code Division Multiple Access
  - Referenced in: Spread-Spectrum-(DSSS-FHSS).md
  - Purpose: Explain CDMA cellular systems
  - Content: Walsh codes, PN sequences, IS-95, WCDMA, near-far problem
  - Related: [[Spread-Spectrum-(DSSS-FHSS)]]

---

## 🔬 Advanced RF Topics

### Propagation & Channel Effects

- **[[Atmospheric Effects]]** - General atmospheric propagation page
  - Referenced in: Free-Space-Path-Loss-(FSPL).md
  - Status: Should redirect to [[Atmospheric-Effects-(Ionospheric,-Tropospheric)]]
  - Action: Update reference to use full page name

### Antenna & Beamforming

- **[[Beamforming (coming soon)]]** - Spatial signal processing basics
  - Referenced in: MIMO-&-Spatial-Multiplexing.md
  - Purpose: Explain how beamforming works
  - Content: Array factor, beam steering, null steering, analog vs digital beamforming
  - Related: [[MIMO-&-Spatial-Multiplexing]], [[Antenna-Theory-Basics]]

- **[[Phased Array Beamforming (coming soon)]]** - Advanced antenna array theory
  - Referenced in: Military-&-Covert-Communications.md
  - Purpose: Explain phased array radar/communications
  - Content: Phase shifters, AESA, beam scanning, grating lobes
  - Related: [[Military-&-Covert-Communications]], [[Antenna-Theory-Basics]]

### Signal Processing

- **[[Adaptive Filters (coming soon)]]** - Adaptive signal processing
  - Referenced in: Military-&-Covert-Communications.md
  - Purpose: Explain adaptive filtering for interference cancellation
  - Content: LMS, RLS, Wiener filters, adaptive equalizers
  - Related: [[Channel-Equalization]]

---

## 🛰️ Navigation & Positioning

- **[[GPS Fundamentals (coming soon)]]** - Civilian GPS basics
  - Referenced in: Military-&-Covert-Communications.md
  - Purpose: Explain GPS C/A code operation
  - Content: Already covered in [[Real-World-System-Examples]] GPS section
  - Action: Consider if separate page needed or update reference

---

## 🔬 Part VIII: Advanced/Speculative Topics

### THz Technology

- **[[THz Bioeffects]]** - Biological effects of terahertz radiation
  - Referenced in: Orchestrated-Objective-Reduction-(Orch-OR).md, Terahertz-(THz)-Technology.md
  - Status: Should redirect to [[THz-Bioeffects-Thermal-and-Non-Thermal]]
  - Action: Update references to use full page name

- **[[Quantum Cascade Lasers (Advanced)]]** - QCL technology deep dive
  - Referenced in: Terahertz-(THz)-Technology.md
  - Purpose: Detailed QCL physics and engineering
  - Content: Band structure, gain medium, waveguide design, fabrication

---

## 🔧 Meta/Documentation Pages

These are example placeholders from the wiki maintenance documentation:

- **[[Display Name]]** - Example from Wiki-Maintenance-Report.md
  - Status: Documentation example, not a real placeholder
  - Action: No action needed

- **[[Topic-Name-With-Hyphens]]** - Example from Wiki-Maintenance-Report.md
  - Status: Documentation example, not a real placeholder
  - Action: No action needed

- **[[wiki links]]** - Example from Quick-Start-Guide.md
  - Status: Generic reference, not a specific page
  - Action: No action needed

---

## 📊 Priority Matrix

### Immediate (Next 1-2 months)
1. **[[Reading the Constellation]]** - High user value, frequently referenced
2. **[[Understanding BER Curves]]** - Complements existing performance analysis pages
3. Fix shorthand redirects (BPSK, FSK, THz Bioeffects, Atmospheric Effects)

### Short Term (3-6 months)
4. **[[CDMA (coming soon)]]** - Completes spread spectrum coverage
5. **[[Beamforming (coming soon)]]** - Natural extension of MIMO page
6. **[[16-QAM]]** - Add as section in QAM page

### Medium Term (6-12 months)
7. **[[Phased Array Beamforming (coming soon)]]** - Advanced military/radar topic
8. **[[Adaptive Filters (coming soon)]]** - Advanced signal processing
9. **[[Quantum Cascade Lasers (Advanced)]]** - Specialized THz topic

### Evaluate Need
10. **[[GPS Fundamentals (coming soon)]]** - May be redundant with Real-World-System-Examples

---

## 📝 Content Guidelines for New Pages

When creating these pages, follow the established wiki format:

### Required Sections
1. **Plain English Explainer** - For non-technical readers (analogies, real-world examples)
2. **Technical Content** - Equations, diagrams, detailed explanations
3. **Related Topics** - Cross-references to other wiki pages
4. **External Resources** - Links to standards, papers, tutorials (see [[Bibliography]])
5. **See Also** - Related wiki pages

### Naming Convention
- Use hyphens for spaces: `New-Page-Name.md`
- Keep parentheses for disambiguation: `Advanced-Topic-(Detailed).md`
- Add to [[Home]] in appropriate Part (I-VIII)
- Update [[Bibliography]] if introducing new references

### Cross-Reference Updates
After creating a page, update all referring pages to use correct link:
- Before: `[[New Page (coming soon)]]`
- After: `[[New-Page-Name]]`

---

## 🔄 Maintenance

**Last Updated**: 2024 (automatically generated from link validation)

**How to Update This Page**:
1. Run link validation script: `python3 /tmp/check_links.py`
2. Review new broken links - are they legitimate placeholders or typos?
3. Add new placeholders to appropriate category above
4. Update priority matrix based on user demand and wiki structure

**Validation Command**:
```bash
# Check for broken links
cd /home/runner/work/chimera/chimera
python3 /tmp/check_links.py
```

---

## 🤝 Contributing

Want to help? Pick a page from the "Immediate" or "Short Term" list and:

1. Create the markdown file: `wiki/Page-Name.md`
2. Follow the content guidelines above
3. Add cross-references from related pages
4. Update this TODO page to mark as complete
5. Submit PR with the new page

See [[Wiki-Maintenance-Report]] for detailed wiki maintenance procedures.

---

## Related Pages

- [[Home]] - Wiki main page with hierarchical structure
- [[Bibliography]] - References for creating new content
- [[Wiki-Maintenance-Report]] - Maintenance procedures and validation tools

---

*This page is automatically updated based on cross-reference validation. Placeholders marked "coming soon" in wiki pages are tracked here.*

# Asset Optimization Guide for Chimera

Guide for optimizing images, fonts, and other assets for production deployment.

## 📦 Current Asset Strategy

Chimera uses Vite's built-in asset optimization which includes:

- ✅ **Automatic hashing** - Cache busting for all assets
- ✅ **Asset bundling** - Small assets inlined as base64
- ✅ **Code splitting** - Separate chunks for vendors
- ✅ **Tree shaking** - Remove unused code
- ✅ **Minification** - Compress JS, CSS, and HTML

---

## 🖼️ Image Optimization

### Supported Formats

Vite automatically handles common image formats:
- PNG (`.png`)
- JPEG/JPG (`.jpg`, `.jpeg`)
- GIF (`.gif`)
- SVG (`.svg`)
- WebP (`.webp`)

### Best Practices

1. **Use SVG for icons and logos**
   ```typescript
   // Inline SVG (recommended for small icons)
   import LogoSvg from '@/assets/logo.svg?raw';
   
   // As URL (for larger images)
   import logoUrl from '@/assets/logo.svg';
   ```

2. **Optimize before committing**
   ```bash
   # For PNG/JPG - use imageoptim, tinypng, or similar
   # Reduce file size by 50-80%
   
   # For SVG - use SVGO
   npx svgo input.svg -o output.svg
   ```

3. **Use appropriate dimensions**
   - Don't use 2000px images for 200px displays
   - Provide 1x and 2x versions for retina displays

4. **Lazy load images**
   ```tsx
   <img 
     src={image} 
     loading="lazy" 
     alt="Description"
   />
   ```

### Automatic Optimization

Small images (< 4KB) are automatically inlined as base64:
```typescript
import smallIcon from './icon.png'; 
// Result: data:image/png;base64,...
```

Larger images are copied to `assets/` with hash:
```typescript
import largeImage from './photo.jpg';
// Result: /assets/photo-[hash].jpg
```

---

## 🔤 Font Optimization

### Font Loading Strategy

1. **Use system fonts when possible**
   ```css
   font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, 
                "Helvetica Neue", Arial, sans-serif;
   ```

2. **Self-host fonts** (preferred over Google Fonts)
   - Add fonts to `chimera-web/public/fonts/`
   - Reference in CSS with `@font-face`
   - Vite will handle hashing automatically

3. **Font display strategy**
   ```css
   @font-face {
     font-family: 'CustomFont';
     src: url('/fonts/custom-font.woff2') format('woff2');
     font-display: swap; /* Show fallback while loading */
     font-weight: 400;
     font-style: normal;
   }
   ```

### Font Formats

Use modern formats for smaller file sizes:
- **WOFF2** (Web Open Font Format 2) - Best compression, modern browsers
- **WOFF** - Fallback for older browsers
- Skip TTF/OTF in production (larger file sizes)

### Subset Fonts

Only include characters you need:
```bash
# Example: Subset font to Latin characters only
pyftsubset font.ttf \
  --output-file=font-subset.woff2 \
  --flavor=woff2 \
  --unicodes=U+0020-007F,U+00A0-00FF
```

### Font Loading Best Practices

1. **Preload critical fonts**
   ```html
   <link rel="preload" href="/fonts/main.woff2" as="font" 
         type="font/woff2" crossorigin>
   ```

2. **Limit font weights**
   - Only include weights you actually use (400, 700)
   - Don't load 9 font weights if you use 2

3. **Use variable fonts**
   - Single file contains multiple weights
   - Often smaller than multiple static fonts

---

## 📄 JavaScript & CSS Optimization

### Automatic Optimizations (Already Configured)

1. **Code Splitting**
   ```
   react-vendor-[hash].js    (~140KB) - React & React-DOM
   state-management-[hash].js (~4KB)  - Zustand
   charts-[hash].js           (~4KB)  - Recharts
   main-[hash].js             (~2KB)  - App code
   ```

2. **Minification with Terser**
   - Removes whitespace
   - Shortens variable names
   - Removes console.log and debugger
   - Dead code elimination

3. **CSS Optimization**
   - Unused CSS removed
   - Minified and combined
   - Code splitting enabled

### Manual Optimizations

1. **Dynamic imports for large components**
   ```tsx
   // Before (loads everything upfront)
   import HeavyChart from './HeavyChart';
   
   // After (loads on demand)
   const HeavyChart = React.lazy(() => import('./HeavyChart'));
   
   <Suspense fallback={<Loading />}>
     <HeavyChart />
   </Suspense>
   ```

2. **Avoid importing entire libraries**
   ```tsx
   // ❌ Bad (imports everything)
   import _ from 'lodash';
   
   // ✅ Good (imports only what you need)
   import debounce from 'lodash/debounce';
   ```

3. **Use production builds of dependencies**
   - Vite automatically does this
   - Smaller, faster React builds

---

## 🎨 CSS Optimization

### Current Setup

- ✅ CSS modules supported
- ✅ PostCSS for autoprefixing
- ✅ CSS code splitting enabled
- ✅ Minification enabled

### Best Practices

1. **Use CSS variables** (defined in `style.css`)
   ```css
   /* ❌ Bad - hardcoded values */
   .button {
     color: #007bff;
     padding: 16px;
   }
   
   /* ✅ Good - use CSS variables */
   .button {
     color: var(--primary-color);
     padding: var(--spacing-md);
   }
   ```

2. **Avoid duplicate styles**
   - Extract common styles to shared classes
   - Use composition over duplication

3. **Critical CSS**
   - Keep above-the-fold CSS small
   - Lazy load non-critical styles

---

## 📊 Bundle Analysis

### Analyze Production Bundle

```bash
cd chimera-web

# Build production
npm run build:prod

# Check total size
du -sh dist-react/

# Check individual chunks
find dist-react/assets -name "*.js" -exec du -h {} \; | sort -rh

# Check CSS files
find dist-react/assets -name "*.css" -exec du -h {} \;
```

### Size Targets

| Asset Type | Target Size | Current |
|-----------|-------------|---------|
| Total Bundle | < 10 MB | 176 KB ✅ |
| Main JS | < 500 KB | 2.26 KB ✅ |
| Vendor JS | < 200 KB | 139 KB ✅ |
| CSS | < 100 KB | 11.5 KB ✅ |

### Warning Signs

- 🚨 Bundle size > 10 MB (CI will warn)
- ⚠️ Single chunk > 1 MB (consider splitting)
- ⚠️ CSS > 100 KB (check for duplicates)
- ⚠️ Vendor chunk > 500 KB (check dependencies)

---

## 🚀 Performance Budget

We enforce a performance budget in CI:

```yaml
# .github/workflows/deploy-react.yml
- name: Check bundle size
  run: |
    TOTAL_SIZE=$(du -sb dist-react/ | cut -f1)
    if [ "$TOTAL_SIZE" -gt 10485760 ]; then  # 10MB
      echo "⚠️ Warning: Bundle size exceeds 10MB"
      exit 1
    fi
```

### Monitoring Bundle Size

1. **Per-commit tracking**
   - CI reports bundle size in summary
   - Compare to previous builds

2. **Size increase alerts**
   - If bundle grows > 20%, investigate
   - Check what dependencies were added

3. **Regular audits**
   - Monthly review of bundle composition
   - Identify optimization opportunities

---

## 🛠️ Tools & Resources

### Analysis Tools

1. **Built-in bundle analyzer**
   ```bash
   npm run build:prod
   # Check output in terminal
   ```

2. **Lighthouse** (in Chrome DevTools)
   ```bash
   # Audit production site
   npm run build:prod
   npm run preview
   # Then run Lighthouse on localhost:4173
   ```

3. **Bundle visualizer** (if needed)
   ```bash
   npm install --save-dev rollup-plugin-visualizer
   # Add to vite.config.prod.ts
   ```

### Optimization Tools

1. **Image optimization**
   - [TinyPNG](https://tinypng.com/) - PNG/JPG compression
   - [SVGO](https://github.com/svg/svgo) - SVG optimization
   - [Squoosh](https://squoosh.app/) - Image conversion

2. **Font optimization**
   - [Font Squirrel](https://www.fontsquirrel.com/tools/webfont-generator) - Webfont generator
   - [glyphhanger](https://github.com/zachleat/glyphhanger) - Font subsetting

3. **CSS optimization**
   - [PurgeCSS](https://purgecss.com/) - Remove unused CSS (if needed)
   - [cssnano](https://cssnano.co/) - CSS minification (built-in)

---

## 📋 Optimization Checklist

Before every major release:

- [ ] Run production build locally
- [ ] Check bundle size (< 10 MB)
- [ ] Analyze chunk composition
- [ ] Test on slow 3G network
- [ ] Run Lighthouse audit (score > 90)
- [ ] Check for unused dependencies
- [ ] Verify all images are optimized
- [ ] Confirm fonts are subset/compressed
- [ ] Review lazy loading opportunities
- [ ] Check for duplicate code

---

## 🎯 Next Steps for Asset Optimization

As the app grows, consider:

1. **Image CDN** - Serve optimized images from CDN
2. **Service Worker** - Cache assets for offline use
3. **Brotli compression** - Better than gzip (GitHub Pages supports this)
4. **Resource hints** - Preload, prefetch, preconnect
5. **HTTP/2 Server Push** - Push critical assets
6. **Progressive image loading** - Blur-up or low-quality placeholders

---

## 📚 Further Reading

- [Vite Asset Handling](https://vitejs.dev/guide/assets.html)
- [Web.dev Performance Guide](https://web.dev/performance/)
- [MDN Image Optimization](https://developer.mozilla.org/en-US/docs/Learn/Performance/Multimedia)
- [Google Fonts Best Practices](https://web.dev/font-best-practices/)

---

**Last Updated**: 2024-01-09  
**Version**: 1.0.0  
**Maintained by**: Chimera Development Team

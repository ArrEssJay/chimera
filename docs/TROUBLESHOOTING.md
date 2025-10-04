# Chimera Troubleshooting Guide

This guide helps resolve common issues when using or developing Chimera.

## 🔧 Installation Issues

### Rust Installation Problems

**Problem**: `rustc` not found or wrong version

**Solution**:
```bash
# Install/update Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update

# Verify version (need 1.70+)
rustc --version

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Node.js/npm Issues

**Problem**: npm install fails with dependency conflicts

**Solution**:
```bash
# Use legacy peer deps flag
npm install --legacy-peer-deps

# Or force resolution
npm install --force

# Clear cache if persistent
npm cache clean --force
rm -rf node_modules package-lock.json
npm install --legacy-peer-deps
```

**Problem**: `npm: command not found`

**Solution**:
```bash
# Install Node.js 18+
# macOS
brew install node

# Or download from https://nodejs.org/

# Verify
node --version  # Should be 18+
npm --version
```

### Trunk Build Issues

**Problem**: `trunk: command not found`

**Solution**:
```bash
# Install trunk
cargo install trunk

# Verify
trunk --version

# Add to PATH if needed
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## 🚀 Runtime Issues

### Application Won't Start

**Problem**: `npm run dev` fails immediately

**Checklist**:
1. Is Node.js 18+ installed? `node --version`
2. Are dependencies installed? `npm install --legacy-peer-deps`
3. Is port 5173 already in use? Try `lsof -i :5173`
4. Check console output for specific error

**Solution**:
```bash
# Kill process on port 5173
lsof -i :5173
kill -9 <PID>

# Or use different port
npm run dev -- --port 3000

# Check for errors
cat /tmp/vite-error.log
```

### WASM Fails to Load

**Problem**: "WebAssembly module could not be compiled" or blank page

**Symptoms**:
- Browser console shows WASM errors
- Blank white page
- Network tab shows 404 for .wasm files

**Solution**:

1. **Clear browser cache**: Hard refresh (Ctrl+Shift+R / Cmd+Shift+R)

2. **Rebuild WASM**:
```bash
cd chimera-core
cargo clean
cargo build --target wasm32-unknown-unknown --release
```

3. **Check MIME types**: WASM files must be served with correct Content-Type

4. **Try different browser**: Chrome/Firefox have best WASM support

5. **Check browser console**: Look for specific error messages

### Simulation Doesn't Run

**Problem**: Clicking "Run Now" does nothing or errors

**Checklist**:
1. Open browser developer tools (F12)
2. Check Console tab for errors
3. Check Network tab for failed requests
4. Verify WASM module loaded

**Common causes**:
- WASM not loaded: See "WASM Fails to Load" above
- Invalid input parameters: Check SNR range, text length
- Browser compatibility: Use Chrome 90+, Firefox 88+, Safari 15.4+

**Solution**:
```javascript
// In browser console, check WASM status
console.log(window.wasmModule);  // Should be defined

// Check simulation state
console.log(localStorage.getItem('chimera-state'));
```

### Audio Not Playing

**Problem**: Audio controls don't produce sound

**Checklist**:
1. Is volume > 0%?
2. Is system audio working?
3. Is browser tab muted? (check tab icon)
4. Are browser audio permissions granted?

**Solution**:

1. **Check audio context**:
```javascript
// In browser console
const audioContext = new AudioContext();
console.log(audioContext.state);  // Should be "running"
```

2. **Enable autoplay**:
   - Chrome: Allow autoplay in site settings
   - Firefox: Preferences → Privacy → Permissions → Autoplay

3. **User interaction requirement**:
   - Audio may need user interaction first
   - Click anywhere on page, then try audio

4. **Browser compatibility**:
   - Safari requires user gesture for audio
   - Try Chrome or Firefox if Safari issues

### Performance Issues

**Problem**: Simulation is very slow or browser freezes

**Symptoms**:
- High CPU usage
- Browser becomes unresponsive
- Simulation takes > 10 seconds

**Solutions**:

1. **Reduce complexity**:
   - Use shorter plaintext (< 50 characters)
   - Disable audio if not needed
   - Close other browser tabs

2. **Check system resources**:
```bash
# Monitor CPU/memory
top
# or
htop
```

3. **Browser optimization**:
   - Enable hardware acceleration (browser settings)
   - Close other tabs/windows
   - Restart browser
   - Update browser to latest version

4. **Development mode overhead**:
   - Production builds are much faster
   - Run `npm run build` and test production version

## 🧪 Development Issues

### Tests Failing

**Problem**: `npm test` shows failures

**Checklist**:
1. Are dependencies installed? `npm install --legacy-peer-deps`
2. Is test database/state clean?
3. Are there leftover test artifacts?

**Solution**:
```bash
# Clean and reinstall
rm -rf node_modules package-lock.json
npm install --legacy-peer-deps

# Clear test cache
npm test -- --clearCache

# Run tests with verbose output
npm test -- --verbose

# Run specific test file
npm test -- Button.test.tsx
```

### Rust Tests Failing

**Problem**: `cargo test` fails

**Common issues**:

1. **Outdated dependencies**:
```bash
cargo update
cargo test
```

2. **Clippy warnings as errors**:
```bash
# Fix clippy issues
cargo clippy --fix --allow-dirty

# Or ignore warnings temporarily
cargo test --lib
```

3. **WASM target issues**:
```bash
# Ensure correct target
rustup target list --installed | grep wasm32
rustup target add wasm32-unknown-unknown

# Test specific target
cargo test --lib --target x86_64-unknown-linux-gnu
```

### Build Errors

**Problem**: `npm run build` fails

**Common errors and solutions**:

1. **TypeScript errors**:
```bash
# Check for type errors
npm run typecheck

# View specific errors
npx tsc --noEmit
```

2. **Vite build fails**:
```bash
# Clear Vite cache
rm -rf node_modules/.vite

# Rebuild
npm run build
```

3. **Out of memory**:
```bash
# Increase Node memory
export NODE_OPTIONS=--max-old-space-size=4096
npm run build
```

### Storybook Issues

**Problem**: `npm run storybook` fails or doesn't show components

**Solution**:
```bash
# Clear Storybook cache
rm -rf node_modules/.cache/storybook

# Reinstall
npm install --legacy-peer-deps

# Run with verbose logging
npm run storybook -- --debug-webpack
```

## 🔍 Debugging Tips

### Browser Developer Tools

**Essential panels**:
1. **Console**: JavaScript errors and logs
2. **Network**: Failed requests, WASM loading
3. **Application**: LocalStorage, Cache
4. **Performance**: CPU/memory profiling

**Useful console commands**:
```javascript
// Check WASM module
console.log(window.wasmModule);

// Inspect React state
console.log(window.__REACT_DEVTOOLS_GLOBAL_HOOK__);

// Check audio context
const ctx = new AudioContext();
console.log('Audio state:', ctx.state);

// Monitor memory
console.memory
```

### React DevTools

Install React DevTools browser extension:
- [Chrome](https://chrome.google.com/webstore/detail/react-developer-tools)
- [Firefox](https://addons.mozilla.org/en-US/firefox/addon/react-devtools/)

**Features**:
- Inspect component hierarchy
- View/edit props and state
- Profile component performance
- Track re-renders

### Rust Debugging

**Add debug output**:
```rust
// Use debug macros
#[cfg(debug_assertions)]
eprintln!("Debug: value = {:?}", value);

// Or web_sys console
use web_sys::console;
console::log_1(&"Message".into());
```

**Run tests with output**:
```bash
cargo test -- --nocapture
```

## 🌐 Browser Compatibility

### Supported Browsers

| Browser | Minimum Version | Notes |
|---------|----------------|-------|
| Chrome | 90+ | ✅ Best support |
| Firefox | 88+ | ✅ Full support |
| Safari | 15.4+ | ⚠️ Audio restrictions |
| Edge | 90+ | ✅ Full support |

### Known Browser Issues

**Safari**:
- Requires user gesture for audio
- WASM compilation may be slower
- LocalStorage quota more restrictive

**Firefox**:
- Console may show WASM warnings (usually safe to ignore)
- Performance slightly behind Chrome

**Mobile browsers**:
- Limited support (desktop recommended)
- Touch interactions may differ
- Audio may not work on iOS Safari

## 📞 Getting More Help

### Before Asking for Help

1. **Check this guide** for your specific issue
2. **Search existing issues** on GitHub
3. **Check browser console** for error messages
4. **Try in different browser** to isolate issue
5. **Test with minimal example** to reproduce

### Reporting Bugs

Include in your report:

```markdown
**Environment**:
- OS: [e.g., macOS 13.0, Ubuntu 22.04]
- Browser: [e.g., Chrome 120, Firefox 115]
- Node version: [e.g., 18.17.0]
- Rust version: [e.g., 1.75.0]

**Steps to Reproduce**:
1. Start dev server
2. Navigate to...
3. Click on...
4. See error

**Expected**: What should happen
**Actual**: What actually happens
**Console output**: [paste errors]
**Screenshots**: [if applicable]
```

### Where to Get Help

- **Documentation**: Check docs/ folder first
- **GitHub Issues**: [Report bugs](https://github.com/ArrEssJay/chimera/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/ArrEssJay/chimera/discussions)
- **Stack Overflow**: Tag with `chimera` and `rust` or `react`

## 🔧 Common Error Messages

### "Failed to fetch dynamically imported module"

**Cause**: WASM file not found or wrong path

**Solution**:
```bash
# Rebuild with clean cache
rm -rf dist node_modules/.vite
npm run build
```

### "Cannot read property of undefined"

**Cause**: State not initialized or async race condition

**Solution**:
- Add null checks: `data?.property`
- Use optional chaining
- Check component mount state

### "Maximum call stack size exceeded"

**Cause**: Infinite loop or recursion

**Solution**:
- Check useEffect dependencies
- Look for circular references
- Add abort conditions to recursive functions

### "Uncaught ReferenceError: process is not defined"

**Cause**: Node.js API used in browser code

**Solution**:
```javascript
// Use environment variable correctly
const isDev = import.meta.env.DEV;  // Not process.env.NODE_ENV
```

---

**Still stuck?** Open a [GitHub Discussion](https://github.com/ArrEssJay/chimera/discussions) with your issue details.

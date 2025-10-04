# Chimera Deployment Guide

This guide covers deploying Chimera to various hosting platforms.

## 🎯 Deployment Options

Chimera can be deployed to:
- Static hosting (Netlify, Vercel, GitHub Pages)
- Cloud platforms (AWS, GCP, Azure)
- Self-hosted servers
- Docker containers

## 📋 Prerequisites

Before deploying:

1. **Build succeeds locally**:
   ```bash
   cd chimera-web
   npm run build
   ```

2. **Tests pass**:
   ```bash
   npm test
   npm run e2e
   ```

3. **Environment variables configured** (if any)

## 🚀 Static Hosting Deployment

### Netlify

**Option 1: Netlify CLI**

```bash
# Install Netlify CLI
npm install -g netlify-cli

# Login
netlify login

# Deploy
cd chimera-web
npm run build
netlify deploy --prod --dir=dist
```

**Option 2: Git Integration**

1. Push code to GitHub/GitLab
2. Connect repository in Netlify dashboard
3. Configure build settings:
   - **Base directory**: `chimera-web`
   - **Build command**: `npm run build`
   - **Publish directory**: `chimera-web/dist`
4. Deploy

**Configuration File** (`netlify.toml`):
```toml
[build]
  base = "chimera-web/"
  command = "npm run build"
  publish = "dist/"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200

[build.environment]
  NODE_VERSION = "18"
```

---

### Vercel

**Option 1: Vercel CLI**

```bash
# Install Vercel CLI
npm install -g vercel

# Login
vercel login

# Deploy
cd chimera-web
vercel --prod
```

**Option 2: Git Integration**

1. Import project in Vercel dashboard
2. Configure:
   - **Framework Preset**: Vite
   - **Root Directory**: `chimera-web`
   - **Build Command**: `npm run build`
   - **Output Directory**: `dist`
3. Deploy

**Configuration File** (`vercel.json`):
```json
{
  "buildCommand": "cd chimera-web && npm run build",
  "outputDirectory": "chimera-web/dist",
  "framework": "vite",
  "rewrites": [
    { "source": "/(.*)", "destination": "/" }
  ]
}
```

---

### GitHub Pages

**Setup**:

1. **Enable GitHub Pages**:
   - Go to repository Settings → Pages
   - Source: GitHub Actions

2. **Create Workflow** (`.github/workflows/deploy.yml`):
```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [main]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - uses: actions/setup-node@v4
        with:
          node-version: 18
          cache: 'npm'
          cache-dependency-path: chimera-web/package-lock.json
      
      - name: Install dependencies
        working-directory: chimera-web
        run: npm ci --legacy-peer-deps
      
      - name: Build
        working-directory: chimera-web
        run: npm run build
      
      - name: Setup Pages
        uses: actions/configure-pages@v4
      
      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: chimera-web/dist
      
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

3. **Update Base Path** (if deploying to subdirectory):
```typescript
// vite.config.ts
export default defineConfig({
  base: '/chimera/', // Match your repo name
  // ... other config
});
```

---

## ☁️ Cloud Platform Deployment

### AWS S3 + CloudFront

**Steps**:

1. **Build the application**:
```bash
cd chimera-web
npm run build
```

2. **Create S3 bucket**:
```bash
aws s3 mb s3://chimera-app
aws s3 website s3://chimera-app --index-document index.html
```

3. **Upload files**:
```bash
aws s3 sync dist/ s3://chimera-app/ --delete
```

4. **Configure CloudFront**:
   - Create CloudFront distribution
   - Origin: S3 bucket
   - Enable HTTPS
   - Set error pages (404 → /index.html)

5. **Automate with CI/CD**:
```yaml
# .github/workflows/deploy-aws.yml
- name: Deploy to S3
  run: |
    aws s3 sync chimera-web/dist/ s3://chimera-app/ --delete
    aws cloudfront create-invalidation --distribution-id ${{ secrets.CF_DIST_ID }} --paths "/*"
  env:
    AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
    AWS_SECRET_ACCESS_KEY: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
```

---

### Google Cloud Platform

**Using Cloud Storage + Cloud CDN**:

1. **Build**:
```bash
cd chimera-web
npm run build
```

2. **Create bucket**:
```bash
gsutil mb gs://chimera-app
gsutil web set -m index.html gs://chimera-app
```

3. **Upload**:
```bash
gsutil -m rsync -r -d dist gs://chimera-app
```

4. **Set permissions**:
```bash
gsutil iam ch allUsers:objectViewer gs://chimera-app
```

---

### Azure Static Web Apps

**Using Azure CLI**:

1. **Create resource**:
```bash
az staticwebapp create \
  --name chimera-app \
  --resource-group myResourceGroup \
  --source https://github.com/yourusername/chimera \
  --location "East US" \
  --branch main \
  --app-location "chimera-web" \
  --output-location "dist"
```

2. **Configuration** (`staticwebapp.config.json`):
```json
{
  "routes": [
    {
      "route": "/*",
      "serve": "/index.html",
      "statusCode": 200
    }
  ],
  "navigationFallback": {
    "rewrite": "/index.html"
  }
}
```

---

## 🐳 Docker Deployment

### Dockerfile

Create `chimera-web/Dockerfile`:

```dockerfile
# Build stage
FROM node:18-alpine AS build

WORKDIR /app

# Copy package files
COPY package*.json ./
RUN npm ci --legacy-peer-deps

# Copy source
COPY . .

# Build application
RUN npm run build

# Production stage
FROM nginx:alpine

# Copy built files
COPY --from=build /app/dist /usr/share/nginx/html

# Copy nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
```

### Nginx Configuration

Create `chimera-web/nginx.conf`:

```nginx
server {
    listen 80;
    server_name _;
    root /usr/share/nginx/html;
    index index.html;

    # Compression
    gzip on;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml text/javascript;

    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # WASM files
    location ~* \.wasm$ {
        types { application/wasm wasm; }
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # SPA fallback
    location / {
        try_files $uri $uri/ /index.html;
    }
}
```

### Build and Run

```bash
# Build image
docker build -t chimera-app chimera-web/

# Run container
docker run -d -p 8080:80 chimera-app

# Or with docker-compose
docker-compose up -d
```

### Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  chimera-web:
    build:
      context: ./chimera-web
      dockerfile: Dockerfile
    ports:
      - "8080:80"
    restart: unless-stopped
    environment:
      - NODE_ENV=production
```

---

## 🔧 Environment Configuration

### Environment Variables

Create `.env.production` in `chimera-web/`:

```env
# API endpoints (if applicable)
VITE_API_URL=https://api.example.com

# Feature flags
VITE_ENABLE_AUDIO=true

# Analytics
VITE_ANALYTICS_ID=UA-XXXXX-Y
```

**Usage in code**:
```typescript
const apiUrl = import.meta.env.VITE_API_URL;
```

---

## ✅ Pre-Deployment Checklist

Before deploying to production:

- [ ] Run full test suite: `npm test`
- [ ] Run E2E tests: `npm run e2e`
- [ ] Check bundle size: `npm run build` (inspect dist/)
- [ ] Test production build locally: `npm run preview`
- [ ] Verify WASM loads correctly
- [ ] Test on multiple browsers
- [ ] Check mobile responsiveness
- [ ] Verify all environment variables set
- [ ] Update version in package.json
- [ ] Update CHANGELOG.md
- [ ] Tag release in git
- [ ] Verify CSP headers (if applicable)
- [ ] Check SSL/HTTPS configuration
- [ ] Test error pages (404, etc.)

---

## 📊 Post-Deployment Verification

After deployment:

1. **Functionality check**:
   - [ ] Application loads
   - [ ] WASM initializes
   - [ ] Simulation runs
   - [ ] Audio plays (if enabled)
   - [ ] All navigation works

2. **Performance check**:
   - [ ] Lighthouse score > 90
   - [ ] First Contentful Paint < 1.8s
   - [ ] Time to Interactive < 3.8s
   - [ ] WASM load time acceptable

3. **Browser testing**:
   - [ ] Chrome (latest)
   - [ ] Firefox (latest)
   - [ ] Safari (latest)
   - [ ] Edge (latest)

4. **Monitor**:
   - Application logs
   - Error rates
   - Performance metrics
   - User feedback

---

## 🔄 CI/CD Pipeline

Example GitHub Actions workflow (`.github/workflows/deploy.yml`):

```yaml
name: Build and Deploy

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - uses: actions/setup-node@v4
        with:
          node-version: 18
      
      - name: Install dependencies
        working-directory: chimera-web
        run: npm ci --legacy-peer-deps
      
      - name: Run tests
        working-directory: chimera-web
        run: npm test
      
      - name: Run E2E tests
        working-directory: chimera-web
        run: npm run e2e

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - uses: actions/setup-node@v4
        with:
          node-version: 18
      
      - name: Install dependencies
        working-directory: chimera-web
        run: npm ci --legacy-peer-deps
      
      - name: Build
        working-directory: chimera-web
        run: npm run build
      
      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: dist
          path: chimera-web/dist

  deploy:
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - name: Download artifact
        uses: actions/download-artifact@v3
        with:
          name: dist
      
      - name: Deploy to production
        run: |
          # Your deployment script here
          echo "Deploying to production..."
```

---

## 🔐 Security Considerations

### Content Security Policy

Add CSP headers in server configuration:

```nginx
add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline';" always;
```

### HTTPS

Always use HTTPS in production:
- Obtain SSL certificate (Let's Encrypt, etc.)
- Redirect HTTP to HTTPS
- Set HSTS header

### Secrets Management

Never commit secrets:
- Use environment variables
- Use secret management services
- Rotate keys regularly

---

## 🆘 Troubleshooting Deployment

### Build Fails

```bash
# Clear cache and rebuild
rm -rf node_modules package-lock.json dist
npm install --legacy-peer-deps
npm run build
```

### WASM Not Loading

- Check MIME types (application/wasm)
- Verify file paths
- Check CORS headers
- Inspect browser console

### Routing Issues

- Ensure SPA fallback configured
- Check base path in vite.config.ts
- Verify server redirects

---

## 📞 Support

For deployment issues:
- Check [Troubleshooting Guide](TROUBLESHOOTING.md)
- Open [GitHub Issue](https://github.com/ArrEssJay/chimera/issues)
- Check hosting platform documentation

---

**Last updated**: October 2025

# Rollback Procedures for Chimera Deployments

Quick reference guide for rolling back deployments in case of issues.

## 🚨 Emergency Rollback (< 5 minutes)

Use when: Critical bug, security issue, or complete site failure.

### Method 1: GitHub UI Revert (Easiest)

1. Go to: https://github.com/ArrEssJay/chimera/commits/main
2. Find the problematic commit
3. Click the **"..."** menu on the right
4. Select **"Revert"**
5. Create and merge the revert PR immediately
6. Deployment happens automatically (~2-3 minutes)

**Pros:**
- ✅ Fastest method
- ✅ No local setup needed
- ✅ Creates proper audit trail

**Cons:**
- ⚠️ Creates new commit (not destructive)

---

### Method 2: Redeploy Previous Version

1. Go to: https://github.com/ArrEssJay/chimera/settings/pages
2. Under **"Build and deployment"**, find deployment history
3. Select previous successful deployment from dropdown
4. Click **"Redeploy"**

**Pros:**
- ✅ Instant rollback
- ✅ No new commits needed

**Cons:**
- ⚠️ Only works if previous deployment still exists
- ⚠️ Temporary fix (next push will re-deploy bad version)

---

### Method 3: Git Revert Command (For Developers)

```bash
# 1. Find the bad commit
git log --oneline -n 10

# 2. Revert the commit (creates new commit)
git revert <bad-commit-hash>

# 3. Push to main
git push origin main

# 4. Wait for auto-deployment (2-3 minutes)
```

**Pros:**
- ✅ Full control
- ✅ Can revert multiple commits

**Cons:**
- ⚠️ Requires local git setup

---

## 📋 Planned Rollback (Non-Critical)

Use when: Issue is discovered but not critical, allows time for investigation.

### Step 1: Assess the Situation

- [ ] Identify what broke (functionality, UI, performance)
- [ ] Determine impact (all users, specific browsers, specific features)
- [ ] Check if it's worth rolling back vs. hotfixing
- [ ] Review CI/CD logs for clues

### Step 2: Choose Rollback Strategy

**Option A: Full Rollback**
- Use if issue is widespread
- Follow Emergency Rollback Method 1 above

**Option B: Hotfix Forward**
- Use if fix is simple and quick
- Create fix on branch
- Test locally
- Merge with fast-track review

**Option C: Feature Flag Disable**
- Use if issue is in new feature
- Disable feature via environment variable
- No deployment needed (if using runtime flags)

### Step 3: Execute Rollback

Follow the chosen method from above.

### Step 4: Post-Rollback Actions

1. **Verify Fix**
   ```bash
   curl -I https://impermanent.io
   # Should return 200 OK
   ```

2. **Test Critical Paths**
   - Visit site and check UI loads
   - Test key user workflows
   - Check browser console for errors

3. **Notify Team**
   - Post in team chat
   - Update incident ticket
   - Document what happened

4. **Root Cause Analysis**
   - Why did the issue occur?
   - Why wasn't it caught in CI?
   - What can prevent it in the future?

---

## 🔍 Verification After Rollback

### Quick Smoke Test

```bash
# 1. Check HTTP status
curl -I https://impermanent.io
# Expected: HTTP/2 200

# 2. Check HTML contains expected content
curl https://impermanent.io | grep "Chimera"
# Expected: Should find "Chimera" in output

# 3. Check JavaScript loads
curl https://impermanent.io | grep -o 'src="/assets/[^"]*\.js"'
# Expected: Should find JS file references
```

### Visual Verification

1. Open https://impermanent.io in browser
2. Check for:
   - ✅ Page loads within 3 seconds
   - ✅ No JavaScript errors in console (F12)
   - ✅ UI displays correctly
   - ✅ Basic interactions work (clicking, typing)
3. Test on multiple browsers:
   - Chrome/Edge
   - Firefox
   - Safari (if available)

### Automated Verification

If you have test suite:
```bash
# Run E2E tests against production
npm run e2e -- --base-url=https://impermanent.io
```

---

## 📊 Rollback Checklist

After any rollback, complete this checklist:

- [ ] Rollback executed successfully
- [ ] Site is accessible (200 OK)
- [ ] UI loads without errors
- [ ] Critical user paths tested
- [ ] Team notified
- [ ] Incident documented
- [ ] Root cause identified
- [ ] Prevention plan created
- [ ] Original PR closed/reverted
- [ ] New issue created for fix

---

## 🛡️ Preventing Rollbacks

### Before Every Deployment

```bash
# Local verification checklist
cd chimera-web

# 1. Type check
npm run typecheck

# 2. Lint
npm run lint

# 3. Run tests
npm run test:coverage
# Verify coverage ≥80%

# 4. Build production
npm run build:prod

# 5. Preview production build
npm run preview
# Test manually at http://localhost:4173

# 6. Check bundle size
du -sh dist-react/
# Should be < 10MB
```

### CI/CD Safety Nets

Our CI pipeline prevents bad deploys:
- ✅ TypeScript compilation must pass
- ✅ ESLint must pass
- ✅ Tests must pass with ≥80% coverage
- ✅ Development build must succeed
- ✅ Production build must succeed
- ✅ Bundle size must be < 10MB

If any fail, deployment is blocked.

---

## 📞 Emergency Contacts

### During Rollback

1. **Check CI/CD Status**: https://github.com/ArrEssJay/chimera/actions
2. **Check GitHub Pages**: https://github.com/ArrEssJay/chimera/settings/pages
3. **Review Logs**: Click on failed workflow → View logs

### Getting Help

- **GitHub Issues**: https://github.com/ArrEssJay/chimera/issues
- **Documentation**: `/docs/DEPLOYMENT.md`
- **Quick Reference**: `/chimera-web/DEPLOYMENT_README.md`

---

## 📝 Common Rollback Scenarios

### Scenario 1: White Screen on Production

**Symptoms:**
- Site loads but shows blank page
- Console shows JavaScript errors

**Quick Fix:**
```bash
# Likely cause: Build issue or missing assets
# Solution: Revert last commit
git revert HEAD
git push origin main
```

**Prevention:**
- Always test production build locally
- Check browser console before deploying

---

### Scenario 2: Slow Performance

**Symptoms:**
- Site loads but very slowly
- Bundle size increased significantly

**Quick Fix:**
```bash
# Check bundle size in last good version
git checkout <last-good-commit>
cd chimera-web && npm run build:prod
du -sh dist-react/

# If new version is much larger, revert
git checkout main
git revert <bad-commit>
git push origin main
```

**Prevention:**
- Monitor bundle size in CI
- Use code splitting
- Lazy load heavy components

---

### Scenario 3: Broken Feature

**Symptoms:**
- Specific feature doesn't work
- Rest of site works fine

**Decision Tree:**
1. Is feature new? → Consider feature flag disable
2. Is feature critical? → Hotfix forward
3. Is fix complex? → Rollback and fix properly

**Hotfix Forward:**
```bash
# Create fix branch
git checkout -b hotfix/fix-feature

# Make minimal fix
# ... edit files ...

# Test locally
npm run typecheck && npm run test && npm run build:prod

# Commit and push
git commit -am "Hotfix: Fix broken feature"
git push origin hotfix/fix-feature

# Create PR and merge immediately
```

---

## 🔄 Rollback Flow Diagram

```
┌─────────────────┐
│ Issue Detected  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐      ┌──────────────┐
│  Is Critical?   │─Yes─→│ Emergency    │
│                 │      │ Rollback     │
└────────┬────────┘      │ (<5 min)     │
         │               └──────────────┘
         No
         │
         ▼
┌─────────────────┐      ┌──────────────┐
│ Is Fix Simple?  │─Yes─→│ Hotfix       │
│                 │      │ Forward      │
└────────┬────────┘      └──────────────┘
         │
         No
         │
         ▼
┌─────────────────┐
│ Planned         │
│ Rollback        │
└─────────────────┘
         │
         ▼
┌─────────────────┐
│ Root Cause      │
│ Analysis        │
└─────────────────┘
         │
         ▼
┌─────────────────┐
│ Fix & Redeploy  │
└─────────────────┘
```

---

## 📈 Post-Rollback Actions

### Immediate (0-1 hour)

1. Verify rollback successful
2. Notify team
3. Document issue
4. Create tracking ticket

### Short-term (1-24 hours)

1. Root cause analysis
2. Create fix
3. Test fix thoroughly
4. Deploy fix

### Long-term (1-7 days)

1. Update prevention measures
2. Improve CI/CD checks
3. Add monitoring
4. Update documentation
5. Share learnings with team

---

## 🎓 Learning from Rollbacks

Every rollback is a learning opportunity:

1. **Document the incident**
   - What happened?
   - What was the impact?
   - How was it discovered?
   - How was it fixed?

2. **Identify root cause**
   - Technical failure?
   - Process failure?
   - Human error?

3. **Implement prevention**
   - Add new tests
   - Improve CI checks
   - Update documentation
   - Add monitoring

4. **Share knowledge**
   - Team retrospective
   - Update runbooks
   - Improve onboarding

---

**Last Updated**: 2024-01-09  
**Version**: 1.0.0  
**Maintained by**: Chimera Development Team

# Chromatic Quick Reference

> **TL;DR**: Visual regression testing for UI components. Every PR gets automatic screenshots. Review and approve changes in Chromatic UI before merge.

## 🎯 What is Chromatic?

Chromatic automatically captures screenshots of all UI components and compares them to baseline images. If there are visual differences, it flags them for review.

## 🚀 Quick Start

### For Developers

**When creating a PR with UI changes:**

1. **Push your code** → Chromatic runs automatically
2. **Check PR status** → Click "Details" on Chromatic check
3. **Review changes** → See visual diffs in browser
4. **Accept or reject** → If intentional, click "Accept". If not, fix and push again.
5. **Merge** → Once accepted, PR can be merged

### For Reviewers

1. **Check Chromatic link** in PR checks
2. **Review visual changes** alongside code
3. **Verify changes match intent** of PR
4. **Accept if correct** or request changes

## 📦 Commands

```bash
# Run Storybook locally
cd chimera-web
npm run storybook

# Build Storybook
npm run build-storybook

# Run Chromatic (requires token)
export CHROMATIC_PROJECT_TOKEN=your_token
npm run chromatic
```

## 🔍 Understanding the UI

### Chromatic Dashboard

When you click the build link in a PR:

- **Green (Passed)**: No visual changes detected
- **Yellow (Changes)**: Visual differences found, needs review
- **Red (Errors)**: Build or rendering errors

### Story States

- **Unchanged**: ✅ Matches baseline
- **Changed**: ⚠️  Visual difference detected
- **New**: 🆕 Story didn't exist in baseline

## 🎨 What Gets Tested?

All Storybook stories are automatically tested:

- **Button**: All variants (primary, secondary, danger), sizes, states
- **Select**: All states (open, closed, disabled, focused)
- **Badge**: All variants and sizes
- **Panel**: All configurations
- **Tooltip**: All positions

## 🤔 Common Questions

### Q: What if I see a change I didn't make?

**A:** This might be:
- An unintended side effect of your changes
- A dependency update that changed styles
- A change from another merged PR

**Fix:** Review the change. If unintentional, investigate and fix.

### Q: What if the change looks correct?

**A:** Click "Accept" in Chromatic UI. The new screenshot becomes the baseline.

### Q: What if I need to make intentional visual changes?

**A:** That's the point! Make your changes, and Chromatic will show them for review. Accept them in the UI.

### Q: Can I test locally before pushing?

**A:** Yes! Run:
```bash
npm run storybook
```
Then check your components visually at http://localhost:6006

### Q: Do I need to run Chromatic locally?

**A:** No, it runs automatically in CI. Local runs are optional.

### Q: What if Chromatic check is pending forever?

**A:** Chromatic is processing screenshots asynchronously. The build has been uploaded successfully. You can merge if all other checks pass. Chromatic will update later.

### Q: How do I add visual tests for a new component?

**A:** Create a `.stories.tsx` file for your component. Chromatic automatically picks it up.

Example:
```tsx
// MyComponent.stories.tsx
import type { Meta, StoryObj } from '@storybook/react';
import { MyComponent } from './MyComponent';

const meta: Meta<typeof MyComponent> = {
  title: 'Components/MyComponent',
  component: MyComponent,
};

export default meta;
type Story = StoryObj<typeof MyComponent>;

export const Default: Story = {
  args: {
    // component props
  },
};
```

## 🚨 When to Investigate

**Investigate if you see:**
- Changes to components you didn't touch
- Large unexpected layout shifts
- Color/font changes you didn't intend
- Missing content or broken layouts

**This is normal if:**
- You changed a component style
- You added a new variant
- You modified spacing/sizing
- You updated a shared CSS variable

## 📚 More Information

- **Setup Guide**: `docs/chromatic-setup.md`
- **Validation**: `docs/chromatic-validation.md`
- **Chromatic Docs**: https://www.chromatic.com/docs/
- **Storybook Guide**: https://storybook.js.org/docs/

## 🆘 Need Help?

1. Check `docs/chromatic-setup.md` for detailed setup info
2. Check workflow logs in GitHub Actions tab
3. Check Chromatic build for error messages
4. Ask in PR comments if you're unsure about a change
5. Open an issue with screenshots if something seems wrong

## 💡 Pro Tips

- **Review changes in Chromatic before code review**: Catch visual issues early
- **Accept changes incrementally**: Don't let too many builds accumulate
- **Write descriptive story names**: Makes it easier to understand what changed
- **Test edge cases**: Create stories for empty states, loading states, errors
- **Keep stories simple**: One story should test one thing

## 🎯 Best Practices

### ✅ Do:
- Review Chromatic results before requesting PR review
- Accept intentional changes promptly
- Investigate unexpected changes
- Add stories for new components
- Test all component states

### ❌ Don't:
- Ignore Chromatic results
- Blindly accept all changes without reviewing
- Skip creating stories for new components
- Merge PRs with unreviewed visual changes
- Forget to check Chromatic on PRs that touch UI

---

**Remember**: Chromatic is here to help catch visual regressions before they reach production. It's a tool to make reviews faster and more thorough, not a blocker!

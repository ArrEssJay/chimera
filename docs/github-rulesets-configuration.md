# GitHub Repository Rulesets Configuration

## Overview

Repository rulesets provide a more flexible and powerful alternative to branch protection rules. They allow you to:
- Apply rules across multiple branches using patterns
- Create reusable rule configurations
- Enforce file-level restrictions
- Set metadata requirements

---

## Ruleset 1: Contract Protection

**Purpose:** Prevent any modifications to contract files without explicit approval

```json
{
  "name": "Contract Protection",
  "target": "branch",
  "enforcement": "active",
  "conditions": {
    "ref_name": {
      "include": ["refs/heads/*"],
      "exclude": []
    }
  },
  "rules": [
    {
      "type": "file_path_restriction",
      "parameters": {
        "restricted_file_paths": [
          "contracts/**"
        ],
        "restrict_mode": "block"
      }
    },
    {
      "type": "pull_request_required",
      "parameters": {
        "required_approving_review_count": 1,
        "dismiss_stale_reviews_on_push": false,
        "require_code_owner_reviews": true,
        "require_last_push_approval": true
      }
    },
    {
      "type": "required_status_checks",
      "parameters": {
        "required_status_checks": [
          {
            "context": "verify-contracts-locked"
          }
        ],
        "strict_required_status_checks_policy": true
      }
    }
  ],
  "bypass_actors": []
}
```

**To create via CLI:**
```bash
gh api repos/ArrEssJay/chimera/rulesets \
  --method POST \
  --input - <<EOF
{
  "name": "Contract Protection",
  "target": "branch",
  "enforcement": "active",
  "conditions": {
    "ref_name": {
      "include": ["refs/heads/*"],
      "exclude": []
    }
  },
  "rules": [
    {
      "type": "file_path_restriction",
      "parameters": {
        "restricted_file_paths": ["contracts/**"]
      }
    }
  ]
}
EOF
```

---

## Ruleset 2: Main Branch Protection

**Purpose:** Enforce quality standards on the main branch

```json
{
  "name": "Main Branch Protection",
  "target": "branch",
  "enforcement": "active",
  "conditions": {
    "ref_name": {
      "include": ["refs/heads/main"],
      "exclude": []
    }
  },
  "rules": [
    {
      "type": "pull_request_required",
      "parameters": {
        "required_approving_review_count": 0,
        "dismiss_stale_reviews_on_push": false,
        "require_code_owner_reviews": false,
        "require_last_push_approval": false
      }
    },
    {
      "type": "required_status_checks",
      "parameters": {
        "required_status_checks": [
          {"context": "detect-conflicts"},
          {"context": "verify-contracts-locked"}
        ],
        "strict_required_status_checks_policy": true
      }
    },
    {
      "type": "required_linear_history"
    },
    {
      "type": "deletion"
    },
    {
      "type": "non_fast_forward"
    },
    {
      "type": "required_signatures"
    }
  ],
  "bypass_actors": [
    {
      "actor_id": 5,
      "actor_type": "RepositoryRole",
      "bypass_mode": "always"
    }
  ]
}
```

---

## Ruleset 3: Feature Branch Workflow

**Purpose:** Allow flexibility for feature branches while maintaining basic quality checks

```json
{
  "name": "Feature Branch Workflow",
  "target": "branch",
  "enforcement": "active",
  "conditions": {
    "ref_name": {
      "include": [
        "refs/heads/feature/*",
        "refs/heads/wave-*"
      ],
      "exclude": []
    }
  },
  "rules": [
    {
      "type": "required_status_checks",
      "parameters": {
        "required_status_checks": [
          {"context": "detect-conflicts"},
          {"context": "verify-contracts-locked"}
        ],
        "strict_required_status_checks_policy": false
      }
    },
    {
      "type": "pull_request_required",
      "parameters": {
        "required_approving_review_count": 0,
        "dismiss_stale_reviews_on_push": false,
        "require_code_owner_reviews": false
      }
    }
  ]
}
```

---

## Ruleset 4: Agent PR Requirements

**Purpose:** Enforce metadata and quality standards for agent-generated PRs

```json
{
  "name": "Agent PR Requirements",
  "target": "branch",
  "enforcement": "active",
  "conditions": {
    "ref_name": {
      "include": ["refs/heads/*"],
      "exclude": ["refs/heads/main"]
    }
  },
  "rules": [
    {
      "type": "commit_message_pattern",
      "parameters": {
        "operator": "starts_with",
        "pattern": "[Wave ",
        "name": "Commit must start with wave identifier",
        "negate": false
      }
    },
    {
      "type": "pull_request_required",
      "parameters": {
        "required_approving_review_count": 0
      }
    },
    {
      "type": "metadata",
      "parameters": {
        "required_labels": [
          "automated"
        ]
      }
    }
  ]
}
```

---

## Ruleset 5: Critical File Protection

**Purpose:** Require explicit approval for infrastructure changes

```json
{
  "name": "Critical File Protection",
  "target": "branch",
  "enforcement": "active",
  "conditions": {
    "ref_name": {
      "include": ["refs/heads/*"],
      "exclude": []
    }
  },
  "rules": [
    {
      "type": "file_path_restriction",
      "parameters": {
        "restricted_file_paths": [
          ".github/workflows/**",
          "Cargo.toml",
          "rust-toolchain.toml",
          "chimera-web/package.json"
        ],
        "restrict_mode": "require_review"
      }
    },
    {
      "type": "pull_request_required",
      "parameters": {
        "required_approving_review_count": 1,
        "require_code_owner_reviews": true
      }
    }
  ]
}
```

---

## Setup Instructions

### Option 1: Via GitHub UI

1. Go to **Repository Settings** → **Rules** → **Rulesets**
2. Click **New ruleset** → **New branch ruleset**
3. Configure each ruleset using the parameters above
4. Set enforcement to **Active**
5. Save the ruleset

### Option 2: Via GitHub CLI

```bash
#!/bin/bash
# setup-rulesets.sh

REPO="ArrEssJay/chimera"

echo "🔒 Creating repository rulesets..."

# 1. Contract Protection
gh api repos/$REPO/rulesets --method POST --input - <<EOF
{
  "name": "Contract Protection",
  "target": "branch",
  "enforcement": "active",
  "conditions": {
    "ref_name": {
      "include": ["refs/heads/*"]
    }
  },
  "rules": [
    {
      "type": "file_path_restriction",
      "parameters": {
        "restricted_file_paths": ["contracts/**"]
      }
    }
  ]
}
EOF

# 2. Main Branch Protection
gh api repos/$REPO/rulesets --method POST --input - <<EOF
{
  "name": "Main Branch Protection",
  "target": "branch",
  "enforcement": "active",
  "conditions": {
    "ref_name": {
      "include": ["refs/heads/main"]
    }
  },
  "rules": [
    {
      "type": "required_linear_history"
    },
    {
      "type": "deletion"
    },
    {
      "type": "non_fast_forward"
    }
  ]
}
EOF

echo "✅ Rulesets created successfully!"
```

### Option 3: Via Terraform (Infrastructure as Code)

```hcl
# terraform/github_rulesets.tf

resource "github_repository_ruleset" "contract_protection" {
  repository = "chimera"
  name       = "Contract Protection"
  target     = "branch"
  enforcement = "active"

  conditions {
    ref_name {
      include = ["~ALL"]
      exclude = []
    }
  }

  rules {
    file_path_restriction {
      restricted_file_paths = ["contracts/**"]
    }
  }
}

resource "github_repository_ruleset" "main_protection" {
  repository = "chimera"
  name       = "Main Branch Protection"
  target     = "branch"
  enforcement = "active"

  conditions {
    ref_name {
      include = ["refs/heads/main"]
      exclude = []
    }
  }

  rules {
    required_linear_history = true
    deletion                = true
    non_fast_forward       = true
    
    pull_request {
      required_approving_review_count = 0
      require_code_owner_reviews     = false
    }
    
    required_status_checks {
      required_check {
        context = "detect-conflicts"
      }
      required_check {
        context = "verify-contracts-locked"
      }
      strict_required_status_checks_policy = true
    }
  }
}
```

---

## Verification

After creating rulesets, verify they're active:

```bash
# List all rulesets
gh api repos/ArrEssJay/chimera/rulesets | jq '.[] | {name, enforcement}'

# Check specific ruleset
gh api repos/ArrEssJay/chimera/rulesets | jq '.[] | select(.name == "Contract Protection")'
```

---

## Ruleset Priority

When multiple rulesets apply to the same branch, they are evaluated in this order:

1. **Contract Protection** (highest priority - blocks contract changes)
2. **Critical File Protection** (requires review for infrastructure)
3. **Main Branch Protection** (enforces main branch standards)
4. **Feature Branch Workflow** (applies to feature branches)
5. **Agent PR Requirements** (metadata requirements)

---

## Bypass Rules

### Repository Admins
- Can bypass rulesets by default
- Configure in ruleset settings: "Bypass list"

### GitHub Apps
- Can bypass specific rulesets
- Required for auto-merge workflows

### Emergency Bypass
```bash
# In case of emergency, temporarily disable ruleset
gh api repos/ArrEssJay/chimera/rulesets/{ruleset_id} \
  --method PATCH \
  --field enforcement=disabled
```

---

## Best Practices

1. **Start with evaluation mode**
   - Set `enforcement: "evaluate"` initially
   - Monitor for false positives
   - Switch to `"active"` after validation

2. **Use specific patterns**
   - Be precise with ref_name patterns
   - Test patterns before deploying

3. **Document bypass scenarios**
   - When should rules be bypassed?
   - Who has bypass authority?

4. **Monitor rule violations**
   - Set up alerts for bypass usage
   - Review violations weekly

5. **Version control rulesets**
   - Store ruleset configs in repo
   - Use PR process for changes

---

## Troubleshooting

### Issue: Ruleset blocks legitimate PR
**Solution:** Add branch to `exclude` list or adjust file patterns

### Issue: Multiple rulesets conflict
**Solution:** Review ruleset priority, consolidate overlapping rules

### Issue: Auto-merge fails due to ruleset
**Solution:** Add GitHub Actions bot to bypass list for specific rules

### Issue: Too many required checks
**Solution:** Use OR logic in status checks or reduce required contexts

---

## Migration Path

If you have existing branch protection rules:

1. **Audit current rules**
   ```bash
   gh api repos/ArrEssJay/chimera/branches/main/protection
   ```

2. **Create equivalent rulesets** (as above)

3. **Test in evaluation mode**
   ```bash
   gh api repos/ArrEssJay/chimera/rulesets/{id} \
     --method PATCH \
     --field enforcement=evaluate
   ```

4. **Monitor for 1 week**

5. **Switch to active mode**
   ```bash
   gh api repos/ArrEssJay/chimera/rulesets/{id} \
     --method PATCH \
     --field enforcement=active
   ```

6. **Remove old branch protection rules**
   ```bash
   gh api repos/ArrEssJay/chimera/branches/main/protection \
     --method DELETE
   ```

---

## Summary

**5 Rulesets Created:**
1. ✅ Contract Protection - Blocks contract modifications
2. ✅ Main Branch Protection - Enforces quality on main
3. ✅ Feature Branch Workflow - Flexible for feature work
4. ✅ Agent PR Requirements - Metadata for agent PRs
5. ✅ Critical File Protection - Review for infrastructure

**Result:** Multi-layered protection that enables safe parallel agent work while protecting critical infrastructure.

# Contributing to CKA Lab

Thank you for contributing! This guide covers how to add new tasks, fix existing ones, and improve the platform.

## Quick Start

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-new-task`
3. Make your changes
4. Submit a pull request

## Adding a New Task

Tasks are YAML files in the `tasks/` directory, organized by domain:

```
tasks/
├── 01_storage/       # Storage tasks (sto-XXX)
├── 02_workloads/     # Workloads & Scheduling tasks (wrk-XXX)
├── 03_networking/    # Services & Networking tasks (net-XXX)
├── 04_troubleshooting/ # Troubleshooting tasks (trb-XXX)
└── 05_cluster_arch/  # Cluster Architecture tasks (arc-XXX)
```

### Task YAML Format

Every task is a YAML file following this schema:

```yaml
id: "net-013"                          # Unique ID: domain prefix + number
domain: "services-networking"          # One of: storage, workloads-scheduling,
                                        #   services-networking, troubleshooting,
                                        #   cluster-architecture
title: "Configure Network Policy"      # Short, descriptive title
description: |
  Multi-line markdown description of what the task requires.
  Be specific about resource names, namespaces, ports, labels.
difficulty: "medium"                    # easy | medium | hard
time_estimate: "7min"                  # Estimated time to complete
weight: 5                              # Points value for scoring
tags:                                  # Relevant tags
  - "network-policy"
  - "security"
hints:                                 # 3 progressive hints
  - "Start by checking what NetworkPolicy resources look like"
  - "Use label selectors to match the source pods"
  - "The policy needs both ingress and egress rules"
exam_tips:                             # Tips relevant to the real CKA exam
  - "NetworkPolicy questions appear frequently"
  - "Remember: default is allow-all unless a policy selects a pod"
solution_files:                        # Paths relative to solutions/ dir
  - "03_networking/net-013-network-policy.yaml"
setup_script: "net-013-setup.sh"       # Optional: script to set up the environment
verify_script: "net-013-verify.sh"     # Optional: script to verify the solution
prerequisites: []                      # Optional: task IDs that should be done first
```

### ID Naming Convention

| Domain | Prefix | Example |
|--------|--------|---------|
| Storage | `sto-` | `sto-001` |
| Workloads & Scheduling | `wrk-` | `wrk-001` |
| Services & Networking | `net-` | `net-001` |
| Troubleshooting | `trb-` | `trb-001` |
| Cluster Architecture | `arc-` | `arc-001` |

Number sequentially within each domain. Check existing files for the next available number.

### Difficulty Guidelines

- **easy** — Single resource creation, simple spec (~3-5 min)
- **medium** — Multi-resource or requires understanding of interactions (~5-10 min)
- **hard** — Multi-step, debugging, or complex configurations (~10-15 min)

### Hint Guidelines

Hints should be progressive:
1. **Hint 1 (nudge)** — Point to the right area/docs without specifics
2. **Hint 2 (clue)** — Mention the specific resource or command
3. **Hint 3 (near-solution)** — Almost gives the answer, just needs assembly

## Solution Files

Place solution YAML files in the `solutions/` directory:

```
solutions/
├── 01_storage/
├── 02_workloads/
├── 03_networking/
├── 04_troubleshooting/
└── 05_cluster_arch/
```

### Solution Requirements

- Must be valid Kubernetes YAML (`kubectl apply -f` should work)
- Should include only the resources the task asks for
- Include a brief comment at the top explaining the approach
- Do NOT include namespace creation unless the task requires it

## Verification Scripts

Verification scripts go in `setup/` and should:

1. Return exit code 0 on success, 1 on failure
2. Print clear feedback (what passed, what failed)
3. Use `kubectl` commands only (no external dependencies)
4. Be idempotent (safe to run multiple times)

Example:

```bash
#!/bin/bash
# verify: net-013

NS="secure-app"
POLICY="deny-all-ingress"

# Check NetworkPolicy exists
if ! kubectl get networkpolicy "$POLICY" -n "$NS" &>/dev/null; then
  echo "FAIL: NetworkPolicy '$POLICY' not found in namespace '$NS'"
  exit 1
fi

# Check policy type is Ingress
TYPE=$(kubectl get networkpolicy "$POLICY" -n "$NS" -o jsonpath='{.spec.policyTypes[0]}')
if [[ "$TYPE" != "Ingress" ]]; then
  echo "FAIL: Expected policyTypes=['Ingress'], got '$TYPE'"
  exit 1
fi

echo "PASS: All checks successful"
exit 0
```

## Code Style

### YAML
- 2-space indentation
- Use string quotes for values that could be ambiguous
- Group related resources in the same file

### Shell Scripts
- Use `#!/bin/bash` shebang
- `set -euo pipefail` for setup scripts
- Meaningful error messages with `echo "ERROR: ..."`
- Clean up on failure with traps

### Rust
- Follow standard Rust formatting (`cargo fmt`)
- Run clippy before committing (`cargo clippy`)
- Add documentation comments for public functions

## Pull Request Process

1. Ensure all new tasks have matching solution files
2. Test solutions against a real cluster if possible
3. Update the task count in README.md if adding to a new domain
4. Keep PRs focused — one task or one feature per PR

## Questions?

Open an issue or reach out:
- [GitHub Issues](https://github.com/simonbbbb/CKA-Hand-on-lab/issues)
- [LinkedIn](https://www.linkedin.com/in/simonbalazshu)

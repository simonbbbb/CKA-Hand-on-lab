<div align="center">

# CKA Lab

**The definitive hands-on CKA exam prep platform**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CKA v1.35](https://img.shields.io/badge/CKA-v1.35-green?logo=kubernetes)](https://www.cncf.io/certification/cka/)
[![Tasks](https://img.shields.io/badge/tasks-55+-cyan)]()

[Get Started](#-quick-start) · [Features](#-features) · [Domains](#-curriculum-domains) · [Landing Page](https://simonbbbb.github.io/CKA-Hand-on-lab/) · [Contributing](CONTRIBUTING.md)

</div>

---

```
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║   /$$$$$$  /$$   /$$  /$$$$$$                                     ║
║  /$$__  $$| $$  /$$/ /$$__  $$                                    ║
║ | $$  \__/| $$ /$$/ | $$  \ $$                                    ║
║ | $$      | $$$$$/  | $$$$$$$$                                    ║
║ | $$      | $$  $$  | $$__  $$                                    ║
║ | $$    $$| $$\  $$ | $$  | $$                                    ║
║ |  $$$$$$/| $$ \  $$| $$  | $$                                    ║
║  \______/ |__/  \__/|__/  |__/                                    ║
║                                                                   ║
║                 CKA Exam Simulator                                ║
║                    by simonbbbb                                   ║
║         Hands-On Lab  ·  Exam Simulator  ·  v2.0                  ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
```

## Why CKA Lab?

Studying for the CKA exam? Most resources give you passive reading or scattered practice questions. **CKA Lab gives you a real, interactive practice environment** with a beautiful terminal UI, auto-verification, and an exam simulator that mirrors the real test.

- **55+ hands-on tasks** aligned to the latest CKA v1.35 curriculum
- **Exam simulator** with a 2-hour timer and realistic question flow
- **Auto-verification** — know instantly if your solution is correct
- **Progressive hints** — 3 levels per task so you never stay stuck
- **Works with any cluster** — minikube, kind, k3s, or cloud

## Features

### Beautiful TUI

A rich terminal interface built with Rust + Ratatui. Navigate domains, browse tasks, track progress — all from your terminal.

```
┌─────────────────────────────────────────────────────────────────┐
│  CKA Lab v2.0  ·  Dashboard                                      │
├──────────────────────┬──────────────────────────────────────────┤
│                      │                                            │
│  ■ Storage (10%)     │  ████████████░░░░  4/5  (80%)            │
│  ■ Workloads (15%)   │  ██████░░░░░░░░░░  6/9  (67%)            │
│  ■ Networking (20%)  │  ████████████████  12/12 (100%) ✓        │
│  ■ Troubleshoot (30%)│  ██████████░░░░░░  8/12 (67%)            │
│  ■ Cluster Arch (25%)│  █████████████░░░  7/9  (78%)            │
│                      │                                            │
│  Overall: 74%        │  [E] Start Exam   [Q] Quit               │
│  ████████████░░░░░░  │                                            │
│                      │                                            │
└──────────────────────┴──────────────────────────────────────────┘
```

### Exam Simulator

Practice under real exam conditions — 120 minutes, 16-17 questions, context switching between nodes.

### Auto-Verification

Run `cka-lab verify` and get instant feedback on every task. No more guessing if your solution is correct.

## Quick Start

### Option 1: Build from source (recommended)

```bash
git clone https://github.com/simonbbbb/CKA-Hand-on-lab.git
cd CKA-Hand-on-lab
cargo run --release
```

### Option 2: Use the bash TUI (no Rust needed)

```bash
git clone https://github.com/simonbbbb/CKA-Hand-on-lab.git
cd CKA-Hand-on-lab/setup
./lab_launcher.sh
```

### Option 3: Just the tasks

Each domain has a `README.md` with tasks and a `solutions/` directory. Read the task, write your YAML, apply it with `kubectl`, verify.

## Curriculum Domains

Aligned with the [official CKA v1.35 curriculum](https://github.com/cncf/curriculum):

| # | Domain | Weight | Tasks | Key Topics |
|---|--------|--------|-------|------------|
| 1 | **Storage** | 10% | 5 | StorageClasses, PV, PVC, dynamic provisioning |
| 2 | **Workloads & Scheduling** | 15% | 9 | Deployments, ConfigMaps, Secrets, HPA, affinity |
| 3 | **Services & Networking** | 20% | 12 | NetworkPolicies, Gateway API, Ingress, CoreDNS |
| 4 | **Troubleshooting** | 30% | 12 | Pod failures, node issues, control plane, networking |
| 5 | **Cluster Architecture** | 25% | 9 | RBAC, CRDs, kubeadm, etcd, Helm, Kustomize, Pod Security |

### What's New in CKA v1.35 (Feb 2025)

This repo is fully updated with the latest curriculum changes:

- **Gateway API** — GatewayClass, Gateway, HTTPRoute (3-5 exam questions expected)
- **Helm & Kustomize** — Package management is now examinable
- **Workload Autoscaling** — HPA and VPA
- **Pod Security Standards** — Replaces deprecated Pod Security Policies
- **Extension Interfaces** — CNI, CSI, CRI understanding

## Directory Structure

```
CKA-Hand-on-lab/
├── src/                      # Rust TUI source code
├── tasks/                    # YAML task definitions (55+ tasks)
│   ├── schema.json           # Task schema for validation
│   ├── 01_storage/
│   ├── 02_workloads/
│   ├── 03_networking/
│   ├── 04_troubleshooting/
│   └── 05_cluster_arch/
├── solutions/                # Official solution files per domain
├── setup/                    # Setup scripts + bash TUI launcher
├── docs/                     # Landing site (GitHub Pages)
├── helper.md                 # Kubernetes & Helm cheat sheet
└── CONTRIBUTING.md           # How to contribute new tasks
```

## Prerequisites

- **Kubernetes cluster** — any of:
  - [Minikube](https://minikube.sigs.k8s.io/) (recommended for local)
  - [kind](https://kind.sigs.k8s.io/)
  - [k3s](https://k3s.io/)
  - Cloud cluster (GKE, AKS, EKS)
- **kubectl** — configured to access your cluster
- **Rust** (for TUI) — `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Environment Setup

### macOS (Apple Silicon)

```bash
brew install qemu minikube kubectl
cd setup && ./reset_lab_environment.sh
```

### macOS / Linux (Docker)

```bash
# Ensure Docker Desktop is running
cd setup && ./reset_lab_environment.sh
```

### Any Cluster

```bash
# Skip minikube setup — just apply resources directly
cd setup && ./01_setup_storage_lab.sh
```

## Workflow

```
┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│  Read Task   │───▶│  Write YAML  │───▶│  kubectl     │───▶│  Verify      │
│  in TUI      │    │  Solution    │    │  apply -f    │    │  Solution    │
└──────────────┘    └──────────────┘    └──────────────┘    └──────────────┘
                                               │                     │
                                               ▼                     ▼
                                        ┌──────────────┐    ┌──────────────┐
                                        │  Check with  │    │  Compare to  │
                                        │  kubectl get │    │  official    │
                                        └──────────────┘    │  solution    │
                                                            └──────────────┘
```

## CKA Exam Tips

<details>
<summary>Time Management</summary>

- 2 hours for 16-17 questions (~7 min per question)
- Skip hard questions, bookmark them, come back
- Start with your strongest domain
- Use `kubectl --dry-run=client -o yaml` to generate YAML templates

</details>

<details>
<summary>Command Shortcuts</summary>

```bash
# Must-know aliases (pre-configured in exam)
alias k=kubectl
source <(kubectl completion bash)

# Generate YAML quickly
kubectl create deployment nginx --image=nginx --dry-run=client -o yaml > deploy.yaml
kubectl expose pod nginx --port=80 --target-port=8080 --dry-run=client -o yaml

# Use kubectl explain instead of docs
kubectl explain pod.spec.containers.resources
kubectl explain deployment.spec.strategy
```

</details>

<details>
<summary>Exam Environment</summary>

- Browser-based terminal via PSI Bridge
- `k` alias and bash completion pre-configured on SSH nodes
- Copy/paste: `Ctrl+Shift+C/V` (not standard Ctrl+C/V)
- Only kubernetes.io/docs is accessible
- Tools available: `jq`, `vim`, `nano`, `curl`, `wget`

</details>

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for:

- How to add new tasks
- Task YAML schema reference
- Solution file guidelines
- Verification script requirements

## Resources

- [Kubernetes Documentation](https://kubernetes.io/docs/home/)
- [CKA Curriculum](https://github.com/cncf/curriculum)
- [CNCF Certification](https://www.cncf.io/certification/cka/)
- [Medium Article](https://medium.com/@balazsdevops/preparing-for-the-new-cka-exam-a-hands-on-lab-environment-00b2b04c3c1f)

## Author

**Simon Balazs**

- [simonbalazs.hu](https://simonbalazs.hu)
- [GitHub](https://github.com/simonbbbb)
- [LinkedIn](https://www.linkedin.com/in/simonbalazshu)
- [Medium](https://medium.com/@balazsdevops)

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**Good luck with your CKA exam!**

If this repo helped you, consider giving it a star.

[![Star History Chart](https://api.star-history.com/svg?repos=simonbbbb/CKA-Hand-on-lab&type=Date)](https://star-history.com/#simonbbbb/CKA-Hand-on-lab&Date)

</div>

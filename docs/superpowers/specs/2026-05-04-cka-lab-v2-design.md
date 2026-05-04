# CKA Lab v2.0 Design Spec

**Date:** 2026-05-04
**Status:** Approved
**Curriculum:** CKA v1.35 (Kubernetes v1.35)

## Overview

Transform CKA-Hand-on-lab from a basic bash-script repo into a world-class CKA exam prep platform with a Rust TUI, static landing site, and comprehensive up-to-date content.

## Architecture

### Subsystem 1: Rust TUI (`src/`)
- **Framework:** Ratatui + Crossterm
- **Binary:** `cka-lab` (single binary distribution)
- **Screens:** Dashboard, Exam Simulator, Lab Browser, Task View, Progress Tracker
- **Features:** Auto-verification, 3-level hint system, exam timer, per-question time tracking
- **Data:** Reads YAML task files from `tasks/` directory

### Subsystem 2: Landing Site (`docs/`)
- **Tech:** Static HTML + Tailwind CSS (CDN)
- **Theme:** Dark terminal aesthetic, monospace fonts, green/cyan accents
- **Sections:** Hero with animated terminal, features grid, domain cards, quick start, contributing
- **Deploy:** GitHub Pages from `docs/` branch/dir

### Subsystem 3: Task Content (`tasks/`)
- **Format:** YAML files with structured schema
- **Schema:** `tasks/schema.json` - validates id, domain, title, difficulty, hints, solution_files, etc.
- **Total target:** ~55 tasks across 5 domains
- **New topics:** Gateway API, advanced NetworkPolicy, HPA/VPA, Pod Security Standards, CNI/CSI debugging

### Subsystem 4: Auto-Verification
- **Rust-based verifier** in the TUI binary
- Runs kubectl checks, scores against expected values, partial credit
- Outputs JSON progress for TUI consumption

## Domain Breakdown (aligned to CKA v1.35)

| Domain | Weight | Existing Tasks | New Tasks | Total |
|--------|--------|---------------|-----------|-------|
| Storage | 10% | 5 | 0 | 5 |
| Workloads & Scheduling | 15% | 7 | 2 (autoscaling) | 9 |
| Services & Networking | 20% | 6 | 6 (Gateway API + NetPol) | 12 |
| Troubleshooting | 30% | 7 | 5 (advanced) | 12 |
| Cluster Architecture | 25% | 7 | 2 (Pod Security) | 9 |
| **Exam-style combined** | - | 0 | 8 | 8 |
| **Total** | | **32** | **23** | **55** |

## File Structure

```
CKA-Hand-on-lab/
├── Cargo.toml
├── src/                          # Rust TUI
│   ├── main.rs
│   ├── app.rs                    # Application state
│   ├── ui/                       # UI components
│   ├── task/                     # Task loading, verification
│   └── exam/                     # Exam simulator
├── tasks/                        # YAML task definitions
│   ├── schema.json
│   ├── 01_storage/
│   ├── 02_workloads/
│   ├── 03_networking/
│   ├── 04_troubleshooting/
│   └── 05_cluster_arch/
├── solutions/                    # Solution YAML files
│   ├── 01_storage/
│   ├── 02_workloads/
│   ├── 03_networking/
│   ├── 04_troubleshooting/
│   └── 05_cluster_arch/
├── setup/                        # Setup & verify scripts
├── docs/                         # Landing site (GitHub Pages)
│   ├── index.html
│   └── assets/
├── img/
├── README.md
├── CONTRIBUTING.md
└── helper.md
```

## Implementation Phases

### Phase 1: Content + Landing Site (immediate)
- Task YAML schema + migrate existing tasks
- New Gateway API, NetworkPolicy, autoscaling, troubleshooting tasks
- Static landing site
- World-class README

### Phase 2: Rust TUI Core
- Project scaffold with ratatui
- Dashboard, Lab Browser, Task View screens
- YAML task loader
- Auto-verification engine

### Phase 3: Exam Simulator + Polish
- Timed exam mode
- Hint system
- Progress persistence
- CI/CD, brew/cargo install

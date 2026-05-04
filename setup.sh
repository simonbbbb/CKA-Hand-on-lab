#!/bin/bash
# CKA Lab - One-Button Setup
# Starts minikube and configures all 5 CKA exam domains
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${BOLD}${CYAN}"
echo "╔════════════════════════════════════════════════════════════╗"
echo "║           CKA Lab - Environment Setup                      ║"
echo "║     One command to get your exam prep lab running          ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# --- Prerequisites ---
check_prereqs() {
    echo -e "${BOLD}${YELLOW}[1/3] Checking prerequisites...${NC}"

    if ! command -v kubectl &>/dev/null; then
        echo -e "${RED}kubectl not found. Installing...${NC}"
        curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/amd64/kubectl"
        chmod +x kubectl && sudo mv kubectl /usr/local/bin/
    fi
    echo -e "  ${GREEN}✓${NC} kubectl: $(kubectl version --client --short 2>/dev/null || kubectl version --client 2>/dev/null | head -1)"

    if ! command -v minikube &>/dev/null; then
        echo -e "${RED}minikube not found. Installing...${NC}"
        if [[ "$(uname)" == "Darwin" ]]; then
            brew install minikube 2>/dev/null || {
                curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-darwin-amd64
                sudo install minikube-darwin-amd64 /usr/local/bin/minikube
                rm minikube-darwin-amd64
            }
        else
            curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64
            sudo install minikube-linux-amd64 /usr/local/bin/minikube
            rm minikube-linux-amd64
        fi
    fi
    echo -e "  ${GREEN}✓${NC} minikube: $(minikube version --short 2>/dev/null)"
}

# --- Start minikube ---
start_minikube() {
    echo ""
    echo -e "${BOLD}${YELLOW}[2/3] Starting minikube...${NC}"

    if minikube status 2>/dev/null | grep -q "Running"; then
        echo -e "  ${GREEN}✓${NC} minikube is already running"
    else
        echo -e "  Starting with 4GB RAM, 2 CPUs..."
        minikube start --memory=4096 --cpus=2 --driver=docker 2>&1 | tail -3
        echo -e "  ${GREEN}✓${NC} minikube started"
    fi

    echo -e "  ${GREEN}✓${NC} cluster: $(kubectl get nodes -o=name | head -1)"
}

# --- Setup all domains ---
setup_domains() {
    echo ""
    echo -e "${BOLD}${YELLOW}[3/3] Setting up lab environments...${NC}"
    echo ""

    local setup_dir="$SCRIPT_DIR/setup"
    local domains=(
        "01_setup_storage_lab.sh|Storage|10%"
        "02_setup_workloads_lab.sh|Workloads & Scheduling|15%"
        "03_setup_networking_lab.sh|Services & Networking|20%"
        "04_setup_troubleshooting_lab.sh|Troubleshooting|30%"
        "05_setup_cluster_arch_lab.sh|Cluster Architecture|25%"
    )

    for entry in "${domains[@]}"; do
        IFS='|' read -r script name weight <<< "$entry"
        echo -e "  ${CYAN}→${NC} Setting up ${BOLD}${name}${NC} (${weight})..."
        if bash "$setup_dir/$script" 2>&1 | sed 's/^/    /'; then
            echo -e "  ${GREEN}✓${NC} ${name} ready"
        else
            echo -e "  ${YELLOW}⚠${NC} ${name} had warnings (non-fatal)"
        fi
        echo ""
    done
}

# --- Summary ---
print_summary() {
    echo -e "${BOLD}${GREEN}"
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║              Setup Complete! Ready to Practice             ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    echo -e "  ${BOLD}Next steps:${NC}"
    echo ""
    echo "  ${CYAN}1.${NC} Start the TUI lab:"
    echo "     ${BOLD}./target/release/cka-lab${NC}"
    echo ""
    echo "  ${CYAN}2.${NC} Or use the classic launcher:"
    echo "     ${BOLD}./setup/lab_launcher.sh${NC}"
    echo ""
    echo -e "  ${CYAN}3.${NC} Verify your setup:"
    echo "     ${BOLD}kubectl get nodes,sc,pv${NC}"
    echo ""
    echo -e "  ${YELLOW}Tip:${NC} Run ${BOLD}./setup.sh --reset${NC} to tear down and start fresh"
    echo ""
}

# --- Reset ---
reset_env() {
    echo -e "${BOLD}${YELLOW}Resetting lab environment...${NC}"
    if minikube status 2>/dev/null | grep -q "Running"; then
        minikube stop
        minikube delete
    fi
    echo -e "${GREEN}Done. Run ./setup.sh to start fresh.${NC}"
    exit 0
}

# --- Main ---
case "${1:-}" in
    --reset|-r)
        reset_env
        ;;
    --help|-h)
        echo "Usage: ./setup.sh [--reset|--help]"
        echo ""
        echo "  (default)  Start minikube and set up all CKA lab domains"
        echo "  --reset    Delete the minikube cluster completely"
        echo "  --help     Show this help"
        exit 0
        ;;
esac

check_prereqs
start_minikube
setup_domains
print_summary

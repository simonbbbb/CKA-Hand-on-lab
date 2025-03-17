#!/bin/bash

# ╔═══════════════════════════════════════════════════════════════╗
# ║ CKA Lab Environment Launcher                                   ║
# ║ -------------------------                                      ║
# ║ A Text-based User Interface (TUI) for managing the CKA Lab     ║
# ║ Environment setup and monitoring resources.                    ║
# ║                                                                ║
# ║ Author: Cascade AI Assistant                                   ║
# ║ Date: March 17, 2025                                           ║
# ║ Version: 1.0                                                   ║
# ║                                                                ║
# ║ Usage:                                                         ║
# ║ ./lab_launcher.sh                                              ║
# ║                                                                ║
# ║ Features:                                                      ║
# ║ - Interactive menu to manage lab environments                  ║
# ║ - Status monitoring for Minikube and namespaces                ║
# ║ - Launch setup scripts for each CKA domain                     ║
# ║ - View resources in each namespace                             ║
# ║ - Monitor node status                                          ║
# ║                                                                ║
# ║ Documentation: See TUI_GUIDE.md for detailed instructions      ║
# ╚═══════════════════════════════════════════════════════════════╝

# ANSI color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color
BOLD='\033[1m'

# Set script directory as working directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

clear_screen() {
    clear
}

print_header() {
    clear_screen
    echo -e "${BOLD}${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BOLD}${BLUE}║           CKA (Certified Kubernetes Administrator)         ║${NC}"
    echo -e "${BOLD}${BLUE}║                  Lab Environment Launcher                  ║${NC}"
    echo -e "${BOLD}${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_status() {
    echo -e "${BOLD}${CYAN}Current Status:${NC}"
    echo -e "  - Minikube: $(minikube status -f '{{.Host}}' 2>/dev/null || echo 'Not Running')"
    echo ""
    echo -e "${BOLD}${CYAN}Available Resources:${NC}"
    echo -e "  Namespaces: $(kubectl get namespaces -o=custom-columns=NAME:.metadata.name --no-headers 2>/dev/null | tr '\n' ', ' | sed 's/,$//' || echo 'No connection to cluster')"
    echo ""
}

launch_setup() {
    local setup_script=$1
    local script_name=$(basename "$setup_script")
    
    print_header
    echo -e "${BOLD}${YELLOW}Launching ${script_name}...${NC}"
    echo ""
    
    bash "$setup_script"
    
    echo ""
    echo -e "${BOLD}${GREEN}Setup Complete!${NC}"
    echo -e "Press Enter to return to the main menu..."
    read
}

check_resources() {
    local namespace=$1
    
    print_header
    echo -e "${BOLD}${YELLOW}Checking resources in namespace: ${namespace}${NC}"
    echo ""
    
    echo -e "${BOLD}${CYAN}Pods:${NC}"
    kubectl get pods -n "$namespace" 2>/dev/null || echo "No pods found or namespace doesn't exist"
    echo ""
    
    echo -e "${BOLD}${CYAN}Services:${NC}"
    kubectl get services -n "$namespace" 2>/dev/null || echo "No services found or namespace doesn't exist"
    echo ""
    
    echo -e "${BOLD}${CYAN}Deployments:${NC}"
    kubectl get deployments -n "$namespace" 2>/dev/null || echo "No deployments found or namespace doesn't exist"
    echo ""
    
    echo -e "Press Enter to return to the main menu..."
    read
}

main_menu() {
    while true; do
        print_header
        print_status
        
        echo -e "${BOLD}${YELLOW}Main Menu:${NC}"
        echo -e "  ${BOLD}${GREEN}1)${NC} Reset Lab Environment"
        echo -e "  ${BOLD}${GREEN}2)${NC} Setup Storage Lab (10%)"
        echo -e "  ${BOLD}${GREEN}3)${NC} Setup Workloads & Scheduling Lab (15%)"
        echo -e "  ${BOLD}${GREEN}4)${NC} Setup Servicing & Networking Lab (20%)"
        echo -e "  ${BOLD}${GREEN}5)${NC} Setup Troubleshooting Lab (30%)"
        echo -e "  ${BOLD}${GREEN}6)${NC} Setup Cluster Architecture Lab (25%)"
        echo -e "  ${BOLD}${YELLOW}Resource Status:${NC}"
        echo -e "  ${BOLD}${GREEN}7)${NC} Check Storage Lab Resources"
        echo -e "  ${BOLD}${GREEN}8)${NC} Check Workloads Lab Resources"
        echo -e "  ${BOLD}${GREEN}9)${NC} Check Networking Lab Resources"
        echo -e "  ${BOLD}${GREEN}10)${NC} Check Troubleshooting Lab Resources"
        echo -e "  ${BOLD}${GREEN}11)${NC} Check Cluster Architecture Lab Resources"
        echo -e "  ${BOLD}${GREEN}12)${NC} View Nodes Status"
        echo -e "  ${BOLD}${RED}0)${NC} Exit"
        echo ""
        echo -en "${BOLD}Enter your choice [0-12]:${NC} "
        read choice
        
        case $choice in
            0)
                clear_screen
                echo -e "${BOLD}${GREEN}Exiting CKA Lab Launcher. Goodbye!${NC}"
                exit 0
                ;;
            1)
                launch_setup "./reset_lab_environment.sh"
                ;;
            2)
                launch_setup "./01_setup_storage_lab.sh"
                ;;
            3)
                launch_setup "./02_setup_workloads_lab.sh"
                ;;
            4)
                launch_setup "./03_setup_networking_lab.sh"
                ;;
            5)
                launch_setup "./04_setup_troubleshooting_lab.sh"
                ;;
            6)
                launch_setup "./05_setup_cluster_arch_lab.sh"
                ;;
            7)
                check_resources "storage"
                ;;
            8)
                check_resources "workloads"
                ;;
            9)
                check_resources "networking"
                ;;
            10)
                check_resources "troubleshooting"
                ;;
            11)
                check_resources "cluster-arch"
                ;;
            12)
                print_header
                echo -e "${BOLD}${YELLOW}Node Status:${NC}"
                echo ""
                kubectl get nodes -o wide
                echo ""
                echo -e "Press Enter to return to the main menu..."
                read
                ;;
            *)
                echo -e "${BOLD}${RED}Invalid choice. Press Enter to continue...${NC}"
                read
                ;;
        esac
    done
}

# Start the main menu
main_menu

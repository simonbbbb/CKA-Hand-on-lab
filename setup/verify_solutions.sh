#!/bin/bash
# CKA Lab - Solution Verification Script

set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo "CKA Lab Solution Verification Script"
echo "===================================="
echo ""

# Function to verify a solution
verify() {
  local section=$1
  local task=$2
  local command=$3
  local expected=$4
  local check_type=$5 # exact, contains, exists
  
  echo -e "${YELLOW}Verifying $section - Task $task${NC}"
  
  # Run the command and capture output
  echo "Running: $command"
  output=$(eval "$command" 2>&1)
  
  # Check the result based on check type
  if [[ "$check_type" == "exact" ]]; then
    if [[ "$output" == "$expected" ]]; then
      echo -e "${GREEN}✓ Success: Task completed correctly${NC}"
      return 0
    else
      echo -e "${RED}✗ Failed: Output doesn't match expected result${NC}"
      echo "Expected: $expected"
      echo "Got: $output"
      return 1
    fi
  elif [[ "$check_type" == "contains" ]]; then
    if [[ "$output" == *"$expected"* ]]; then
      echo -e "${GREEN}✓ Success: Task completed correctly${NC}"
      return 0
    else
      echo -e "${RED}✗ Failed: Output doesn't contain expected result${NC}"
      echo "Expected to contain: $expected"
      echo "Got: $output"
      return 1
    fi
  elif [[ "$check_type" == "exists" ]]; then
    if [[ "$output" != *"not found"* && "$output" != *"No resources found"* ]]; then
      echo -e "${GREEN}✓ Success: Resource exists${NC}"
      return 0
    else
      echo -e "${RED}✗ Failed: Resource doesn't exist${NC}"
      echo "Got: $output"
      return 1
    fi
  fi
}

# Main menu
show_menu() {
  echo "Select a section to verify:"
  echo "1) Storage"
  echo "2) Workloads and Scheduling"
  echo "3) Servicing and Networking"
  echo "4) Troubleshooting"
  echo "5) Cluster Architecture"
  echo "q) Quit"
  read -p "Enter choice [1-5 or q]: " choice
  
  case $choice in
    1) verify_storage ;;
    2) verify_workloads ;;
    3) verify_networking ;;
    4) verify_troubleshooting ;;
    5) verify_cluster_architecture ;;
    q) exit 0 ;;
    *) echo "Invalid choice"; show_menu ;;
  esac
}

verify_storage() {
  echo -e "\n${YELLOW}Verifying Storage Tasks${NC}"
  
  # Task 1: Create a StorageClass
  verify "Storage" "1" "kubectl get sc fast-storage -o jsonpath='{.provisioner}{\" \"}{.volumeBindingMode}{\" \"}{.reclaimPolicy}'" "kubernetes.io/no-provisioner WaitForFirstConsumer Delete" "exact"
  
  # Task 2: Create a Persistent Volume
  verify "Storage" "2" "kubectl get pv pv-manual -o jsonpath='{.spec.capacity.storage}{\" \"}{.spec.accessModes[0]}{\" \"}{.spec.hostPath.path}{\" \"}{.spec.storageClassName}'" "1Gi ReadWriteOnce /mnt/data manual" "exact"
  
  # Task 3: Create a Persistent Volume Claim
  verify "Storage" "3" "kubectl get pvc pvc-manual -o jsonpath='{.spec.resources.requests.storage}{\" \"}{.spec.accessModes[0]}{\" \"}{.spec.storageClassName}'" "500Mi ReadWriteOnce manual" "exact"
  
  # Task 4: Create a Pod that uses the PVC
  verify "Storage" "4" "kubectl get pod pod-with-pvc -o jsonpath='{.spec.containers[0].image}{\" \"}{.spec.volumes[0].persistentVolumeClaim.claimName}'" "nginx pvc-manual" "exact"
  
  # Task 5: Implement Dynamic Volume Provisioning
  # This is a bit more complex and depends on the environment capabilities
  # We'll just check if they created a PVC with the default StorageClass
  verify "Storage" "5" "kubectl get pvc -l task=dynamic-provisioning" "" "exists"
  
  show_menu
}

verify_workloads() {
  echo -e "\n${YELLOW}Verifying Workloads and Scheduling Tasks${NC}"
  
  # Example tasks - replace with actual task verifications
  verify "Workloads" "1" "kubectl get deployment custom-deployment" "" "exists"
  verify "Workloads" "2" "kubectl get pod -l app=pod-affinity" "" "exists"
  
  show_menu
}

verify_networking() {
  echo -e "\n${YELLOW}Verifying Servicing and Networking Tasks${NC}"
  
  # Example tasks - replace with actual task verifications
  verify "Networking" "1" "kubectl get service custom-service" "" "exists"
  verify "Networking" "2" "kubectl get ingress custom-ingress" "" "exists"
  
  show_menu
}

verify_troubleshooting() {
  echo -e "\n${YELLOW}Verifying Troubleshooting Tasks${NC}"
  
  # Example tasks - replace with actual task verifications
  verify "Troubleshooting" "1" "kubectl get pods -n troubleshooting broken-pod" "" "exists"
  verify "Troubleshooting" "2" "kubectl get events --field-selector involvedObject.name=resource-constrained-pod -n troubleshooting" "" "exists"
  
  show_menu
}

verify_cluster_architecture() {
  echo -e "\n${YELLOW}Verifying Cluster Architecture Tasks${NC}"
  
  # Example tasks - replace with actual task verifications
  verify "Cluster Architecture" "1" "kubectl get role custom-role -n rbac-test" "" "exists"
  verify "Cluster Architecture" "2" "kubectl get rolebinding custom-binding -n rbac-test" "" "exists"
  
  show_menu
}

# Start the script
show_menu

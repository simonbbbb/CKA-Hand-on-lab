#!/bin/bash
# CKA Lab - Solution Verification Script

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo "CKA Lab Solution Verification Script"
echo "===================================="
echo ""

# Track total checks and failures
total_checks=0
failed_checks=0

# Function to verify a solution
verify() {
  local section=$1
  local task=$2
  local command=$3
  local expected=$4
  local check_type=$5 # exact, contains, exists
  
  echo -e "${YELLOW}Verifying $section - Task $task${NC}"
  
  # Increment total checks
  ((total_checks++))
  
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
      ((failed_checks++))
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
      ((failed_checks++))
      return 1
    fi
  elif [[ "$check_type" == "exists" ]]; then
    if [[ "$output" != *"not found"* && "$output" != *"No resources found"* ]]; then
      echo -e "${GREEN}✓ Success: Resource exists${NC}"
      return 0
    else
      echo -e "${RED}✗ Failed: Resource doesn't exist${NC}"
      echo "Got: $output"
      ((failed_checks++))
      return 1
    fi
  fi
}

# Function to display verification summary
display_summary() {
  echo -e "\n${YELLOW}Verification Summary${NC}"
  echo -e "Total checks: $total_checks"
  echo -e "Failed checks: $failed_checks"
  
  if [ $failed_checks -eq 0 ]; then
    echo -e "${GREEN}All checks passed successfully!${NC}"
  else
    echo -e "${RED}Some checks failed. Review the output above for details.${NC}"
  fi
  
  # Reset counters for next verification section
  total_checks=0
  failed_checks=0
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
  
  # Reset counters for this section
  total_checks=0
  failed_checks=0
  
  # Task 1: Create a StorageClass
  verify "Storage" "1" "kubectl get sc fast-storage -o jsonpath='{.provisioner}{\" \"}{.volumeBindingMode}{\" \"}{.reclaimPolicy}'" "kubernetes.io/no-provisioner WaitForFirstConsumer Delete" "exact"
  
  # Task 2: Create a Persistent Volume
  verify "Storage" "2" "kubectl get pv pv-manual -o jsonpath='{.spec.capacity.storage}{\" \"}{.spec.accessModes[0]}{\" \"}{.spec.hostPath.path}{\" \"}{.spec.storageClassName}'" "1Gi ReadWriteOnce /mnt/data manual" "exact"
  
  # Task 3: Create a Persistent Volume Claim
  verify "Storage" "3" "kubectl get pvc pvc-manual -o jsonpath='{.spec.resources.requests.storage}{\" \"}{.spec.accessModes[0]}{\" \"}{.spec.storageClassName}'" "500Mi ReadWriteOnce manual" "exact"
  
  # Task 4: Create a Pod that uses the PVC
  verify "Storage" "4" "kubectl get pod pod-with-pvc -o jsonpath='{.spec.containers[0].image}{\" \"}{.spec.volumes[0].persistentVolumeClaim.claimName}'" "nginx pvc-manual" "exact"
  
  # Task 5: Implement Dynamic Volume Provisioning
  verify "Storage" "5" "kubectl get pvc -l task=dynamic-provisioning -o jsonpath='{.items[0].spec.storageClassName}'" "standard" "contains"
  
  # Display summary for this section
  display_summary
  
  show_menu
}

verify_workloads() {
  echo -e "\n${YELLOW}Verifying Workloads and Scheduling Tasks${NC}"
  
  # Reset counters for this section
  total_checks=0
  failed_checks=0
  
  # Task 1: Create a Deployment with specific requirements
  verify "Workloads" "1" "kubectl get deployment custom-deployment -o jsonpath='{.spec.replicas}{\" \"}{.spec.template.spec.containers[0].image}'" "3 nginx:latest" "exact"
  
  # Task 2: Configure Pod with Node Affinity
  verify "Workloads" "2" "kubectl get pod -l app=pod-affinity -o jsonpath='{.items[0].spec.affinity.nodeAffinity.requiredDuringSchedulingIgnoredDuringExecution.nodeSelectorTerms[0].matchExpressions[0].key}'" "disk" "exact"
  
  # Task 3: Create a Horizontal Pod Autoscaler
  verify "Workloads" "3" "kubectl get hpa -o jsonpath='{.items[0].spec.minReplicas}{\" \"}{.items[0].spec.maxReplicas}{\" \"}{.items[0].spec.targetCPUUtilizationPercentage}'" "1 10 50" "contains"
  
  # Task 4: Configure with ConfigMap and Secret
  verify "Workloads" "4.1" "kubectl get configmap -o jsonpath='{.items[*].metadata.name}'" "app-config" "contains"
  verify "Workloads" "4.2" "kubectl get secret -o jsonpath='{.items[*].metadata.name}'" "app-secrets" "contains"
  verify "Workloads" "4.3" "kubectl get pod -l config=mounted -o jsonpath='{.items[0].spec.containers[0].env[0].valueFrom.configMapKeyRef.name}'" "app-config" "contains"
  
  # Task 5: Create a DaemonSet
  verify "Workloads" "5" "kubectl get ds -o jsonpath='{.items[0].metadata.name}{\" \"}{.items[0].spec.template.spec.containers[0].image}'" "monitoring-agent fluentd:latest" "contains"
  
  # Display summary for this section
  display_summary
  
  show_menu
}

verify_networking() {
  echo -e "\n${YELLOW}Verifying Servicing and Networking Tasks${NC}"
  
  # Reset counters for this section
  total_checks=0
  failed_checks=0
  
  # Task 1: Create ClusterIP Service
  verify "Networking" "1" "kubectl get service custom-service -n networking-test -o jsonpath='{.spec.type}{\" \"}{.spec.selector.app}'" "ClusterIP frontend" "exact"
  
  # Task 2: Create NetworkPolicy
  verify "Networking" "2" "kubectl get networkpolicy -n networking-test -o jsonpath='{.items[0].spec.podSelector.matchLabels.app}{\" \"}{.items[0].spec.ingress[0].from[0].podSelector.matchLabels.app}'" "backend frontend" "contains"
  
  # Task 3: Create Ingress resource
  verify "Networking" "3" "kubectl get ingress -n networking-test -o jsonpath='{.items[0].spec.rules[0].http.paths[0].path}{\" \"}{.items[0].spec.rules[0].http.paths[0].backend.service.name}'" "/ frontend-service" "contains"
  
  # Task 4: Configure DNS resolution
  verify "Networking" "4" "kubectl get pod -n networking-test -l task=dns-resolution -o jsonpath='{.items[0].spec.containers[0].env[0].name}'" "SERVICE_URL" "contains"
  
  # Task 5: Create NodePort service
  verify "Networking" "5" "kubectl get service -n networking-test -o jsonpath='{.items[*].spec.type}'" "NodePort" "contains"
  
  # Display summary for this section
  display_summary
  
  show_menu
}

verify_troubleshooting() {
  echo -e "\n${YELLOW}Verifying Troubleshooting Tasks${NC}"
  
  # Reset counters for this section
  total_checks=0
  failed_checks=0
  
  # Task 1: Fix broken pod
  verify "Troubleshooting" "1" "kubectl get pod broken-pod -n troubleshooting -o jsonpath='{.status.phase}'" "Running" "exact"
  
  # Task 2: Fix resource-constrained pod
  verify "Troubleshooting" "2" "kubectl get pod resource-constrained-pod -n troubleshooting -o jsonpath='{.status.phase}'" "Running" "exact"
  
  # Task 3: Fix service selector issue
  verify "Troubleshooting" "3" "kubectl get endpoints frontend-service -n troubleshooting -o jsonpath='{.subsets[0].addresses}'" "" "exists"
  
  # Task 4: Fix ConfigMap error
  verify "Troubleshooting" "4" "kubectl get configmap app-config -n troubleshooting -o jsonpath='{.data.DATABASE_URL}'" "mysql://user:password@db:3306/app" "exact"
  
  # Task 5: Fix deployment update strategy
  verify "Troubleshooting" "5" "kubectl get deployment web-app -n troubleshooting -o jsonpath='{.spec.strategy.type}'" "RollingUpdate" "exact"
  
  # Display summary for this section
  display_summary
  
  show_menu
}

verify_cluster_architecture() {
  echo -e "\n${YELLOW}Verifying Cluster Architecture Tasks${NC}"
  
  # Reset counters for this section
  total_checks=0
  failed_checks=0
  
  # Task 1: Create RBAC Role
  verify "Cluster Architecture" "1" "kubectl get role custom-role -n rbac-test -o jsonpath='{.rules[0].resources[0]}{\" \"}{.rules[0].verbs[0]}'" "pods get" "contains"
  
  # Task 2: Create RBAC RoleBinding
  verify "Cluster Architecture" "2" "kubectl get rolebinding custom-binding -n rbac-test -o jsonpath='{.subjects[0].kind}{\" \"}{.subjects[0].name}{\" \"}{.roleRef.name}'" "User jane custom-role" "exact"
  
  # Task 3: Create a CRD
  verify "Cluster Architecture" "3" "kubectl get crd -o jsonpath='{.items[*].metadata.name}'" "backups.cka.training" "contains"
  
  # Task 4: Check kubeadm config file
  # For this task, we can check if the file exists with correct structure
  # Since we can't directly test kubeadm in a lab, we'll check file content
  if [ -f "/Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/user_solutions/kubeadm-config.yaml" ]; then
    verify "Cluster Architecture" "4" "grep -c 'kind: ClusterConfiguration' /Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/user_solutions/kubeadm-config.yaml" "1" "contains"
  else
    echo -e "${YELLOW}Task 4: kubeadm configuration file not found in user_solutions directory${NC}"
    echo -e "${YELLOW}Please create the file to pass this verification${NC}"
    ((failed_checks++))
    ((total_checks++))
  fi
  
  # Task 5: ETCD Backup procedure
  # For this task, we can check if the file exists with correct commands
  if [ -f "/Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/user_solutions/etcd-backup.md" ]; then
    verify "Cluster Architecture" "5" "grep -c 'ETCDCTL_API=3 etcdctl' /Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/user_solutions/etcd-backup.md" "1" "contains"
  else
    echo -e "${YELLOW}Task 5: ETCD backup procedure not found in user_solutions directory${NC}"
    echo -e "${YELLOW}Please create the file to pass this verification${NC}"
    ((failed_checks++))
    ((total_checks++))
  fi
  
  # Task 6: Helm Chart Installation
  # Check if there's a values.yaml file for Helm
  if [ -f "/Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/user_solutions/values.yaml" ]; then
    verify "Cluster Architecture" "6" "grep -c 'image:' /Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/user_solutions/values.yaml" "1" "contains"
  else
    echo -e "${YELLOW}Task 6: Helm chart values file not found in user_solutions directory${NC}"
    echo -e "${YELLOW}Please create a values.yaml file to pass this verification${NC}"
    ((failed_checks++))
    ((total_checks++))
  fi
  
  # Task 7: Kustomize Configuration
  # Check if there's a kustomization.yaml file
  if [ -d "/Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/user_solutions/kustomize" ]; then
    verify "Cluster Architecture" "7" "grep -c 'kustomize' /Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/user_solutions/kustomize/kustomization.yaml" "1" "contains"
  else
    echo -e "${YELLOW}Task 7: Kustomize directory not found in user_solutions directory${NC}"
    echo -e "${YELLOW}Please create a kustomize directory with kustomization.yaml to pass this verification${NC}"
    ((failed_checks++))
    ((total_checks++))
  fi
  
  # Display summary for this section
  display_summary
  
  show_menu
}

# Start the script
show_menu

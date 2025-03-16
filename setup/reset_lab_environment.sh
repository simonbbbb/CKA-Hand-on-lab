#!/bin/bash
# CKA Lab - Reset Lab Environment

set -e

echo "Resetting CKA Lab Environment..."

# Check if minikube is running
if minikube status | grep -q "Running"; then
  echo "Stopping Minikube..."
  minikube stop
  
  echo "Deleting Minikube cluster..."
  minikube delete
  
  echo "Starting a fresh Minikube instance..."
  minikube start --memory=4096 --cpus=2 --driver=qemu2
else
  echo "Minikube is not running. Starting a fresh instance..."
  minikube start --memory=4096 --cpus=2 --driver=qemu2
fi

# Clean up any existing resources
echo "Cleaning up any existing resources..."
kubectl delete namespace workloads-test networking-test troubleshooting rbac-test 2>/dev/null || true

echo "Lab environment has been reset successfully!"
echo "You can now run the individual setup scripts for each section."

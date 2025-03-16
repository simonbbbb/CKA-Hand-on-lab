#!/bin/bash
# CKA Lab - Storage Environment Setup Script

set -e

echo "Setting up Storage Lab Environment..."

# Check if minikube is running
if ! minikube status | grep -q "Running"; then
  echo "Minikube is not running. Starting minikube..."
  minikube start --memory=4096 --cpus=2 --driver=qemu2
fi

# Create directory for hostPath volumes
echo "Creating directory for hostPath volumes..."
minikube ssh "sudo mkdir -p /mnt/data"
minikube ssh "sudo chmod 777 /mnt/data"
minikube ssh "echo 'This is test data for CKA exam practice' | sudo tee /mnt/data/test-file.txt"

# Create the manual StorageClass
echo "Creating manual StorageClass..."
cat <<EOF | kubectl apply -f -
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: manual
provisioner: kubernetes.io/no-provisioner
volumeBindingMode: WaitForFirstConsumer
EOF

# Create another StorageClass for different scenarios
echo "Creating additional StorageClass..."
cat <<EOF | kubectl apply -f -
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: fast-storage
provisioner: kubernetes.io/no-provisioner
volumeBindingMode: WaitForFirstConsumer
reclaimPolicy: Delete
EOF

# Create a PersistentVolume for practice
echo "Creating a sample PersistentVolume..."
cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: PersistentVolume
metadata:
  name: sample-pv
spec:
  capacity:
    storage: 1Gi
  accessModes:
    - ReadWriteOnce
  persistentVolumeReclaimPolicy: Retain
  storageClassName: manual
  hostPath:
    path: "/mnt/data"
EOF

echo "Storage lab environment setup complete!"
echo ""
echo "Available resources:"
echo "- StorageClass: manual"
echo "- StorageClass: fast-storage"
echo "- PersistentVolume: sample-pv"
echo ""
echo "You can now proceed with the storage tasks in the Storage section."
echo "To verify the setup, run: kubectl get sc,pv"

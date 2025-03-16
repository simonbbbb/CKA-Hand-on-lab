#!/bin/bash
# CKA Lab - Cluster Architecture, Installation, and Configuration Environment Setup Script

set -e

echo "Setting up Cluster Architecture Lab Environment..."

# Check if minikube is running
if ! minikube status | grep -q "Running"; then
  echo "Minikube is not running. Starting minikube..."
  minikube start --memory=4096 --cpus=2 --driver=qemu2
fi

# Create namespace for RBAC exercises
echo "Creating namespace for RBAC exercises..."
kubectl create namespace rbac-test

# Create service accounts for RBAC exercises
echo "Creating service accounts for RBAC exercises..."
kubectl create serviceaccount view-user -n rbac-test
kubectl create serviceaccount edit-user -n rbac-test
kubectl create serviceaccount admin-user -n rbac-test

# Setup Helm for package management exercises
echo "Setting up Helm..."
if ! command -v helm &> /dev/null; then
    echo "Helm not found. Please install Helm manually:"
    echo "For macOS: brew install helm"
    echo "For more information, visit: https://helm.sh/docs/intro/install/"
else
    echo "Helm is already installed."
    helm repo add stable https://charts.helm.sh/stable
    helm repo update
fi

# Create a Custom Resource Definition for practice
echo "Creating a Custom Resource Definition for practice..."
cat <<EOF | kubectl apply -f -
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: backups.stable.example.com
spec:
  group: stable.example.com
  versions:
    - name: v1
      served: true
      storage: true
      schema:
        openAPIV3Schema:
          type: object
          properties:
            spec:
              type: object
              properties:
                storageType:
                  type: string
                frequency:
                  type: string
  scope: Namespaced
  names:
    plural: backups
    singular: backup
    kind: Backup
    shortNames:
    - bkp
EOF

# Create Custom Resource instance
echo "Creating a Custom Resource instance..."
cat <<EOF | kubectl apply -f -
apiVersion: stable.example.com/v1
kind: Backup
metadata:
  name: database-backup
  namespace: rbac-test
spec:
  storageType: S3
  frequency: daily
EOF

# Create different role bindings for RBAC practice
echo "Creating role bindings for RBAC practice..."

# Create read-only role
cat <<EOF | kubectl apply -f -
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  namespace: rbac-test
  name: pod-reader
rules:
- apiGroups: [""]
  resources: ["pods"]
  verbs: ["get", "watch", "list"]
EOF

# Bind role to view-user
cat <<EOF | kubectl apply -f -
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: read-pods
  namespace: rbac-test
subjects:
- kind: ServiceAccount
  name: view-user
  namespace: rbac-test
roleRef:
  kind: Role
  name: pod-reader
  apiGroup: rbac.authorization.k8s.io
EOF

# Create a sample deployment with multiple replicas to explore kubeadm commands
echo "Creating sample deployment for cluster management practice..."
kubectl create deployment nginx-deploy --image=nginx:latest --replicas=3 -n rbac-test

# Create instructions for kubeadm practice (as we can't actually run these commands in Minikube)
cat <<EOF > /Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/kubeadm_practice_guide.md
# Kubeadm Practice Guide

Since we're using Minikube for our lab environment, we can't directly practice with kubeadm.
However, here are the commands you would use in a real-world scenario:

## 1. Initialize a new Kubernetes cluster

\`\`\`bash
# Initialize the control plane
sudo kubeadm init --pod-network-cidr=10.244.0.0/16 --kubernetes-version=v1.28.0

# Set up kubeconfig for the user
mkdir -p $HOME/.kube
sudo cp -i /etc/kubernetes/admin.conf $HOME/.kube/config
sudo chown $(id -u):$(id -g) $HOME/.kube/config
\`\`\`

## 2. Install a pod network add-on (example with Calico)

\`\`\`bash
kubectl apply -f https://docs.projectcalico.org/manifests/calico.yaml
\`\`\`

## 3. Join worker nodes to the cluster

\`\`\`bash
# On worker nodes, run the join command that was output by kubeadm init
sudo kubeadm join <control-plane-host>:<control-plane-port> --token <token> --discovery-token-ca-cert-hash sha256:<hash>
\`\`\`

## 4. Create a high-availability cluster

\`\`\`bash
# First set up a load balancer for the API server
# Then initialize the first control plane node
sudo kubeadm init --control-plane-endpoint "LOAD_BALANCER_DNS:LOAD_BALANCER_PORT" --upload-certs --pod-network-cidr=10.244.0.0/16

# Join additional control plane nodes
sudo kubeadm join LOAD_BALANCER_DNS:LOAD_BALANCER_PORT --token <token> --discovery-token-ca-cert-hash sha256:<hash> --control-plane --certificate-key <certificate-key>
\`\`\`

## 5. Upgrade a cluster

\`\`\`bash
# Upgrade kubeadm
sudo apt-get update && sudo apt-get install -y kubeadm=1.29.0-00

# Check upgrade plan
sudo kubeadm upgrade plan

# Apply the upgrade
sudo kubeadm upgrade apply v1.29.0

# Upgrade kubelet and kubectl
sudo apt-get update && sudo apt-get install -y kubelet=1.29.0-00 kubectl=1.29.0-00
sudo systemctl daemon-reload
sudo systemctl restart kubelet
\`\`\`

## 6. Backup and restore etcd

\`\`\`bash
# Backup etcd
sudo ETCDCTL_API=3 etcdctl --endpoints=https://127.0.0.1:2379 \\
  --cacert=/etc/kubernetes/pki/etcd/ca.crt \\
  --cert=/etc/kubernetes/pki/etcd/server.crt \\
  --key=/etc/kubernetes/pki/etcd/server.key \\
  snapshot save /tmp/etcd-backup.db

# Restore etcd from backup
sudo ETCDCTL_API=3 etcdctl --endpoints=https://127.0.0.1:2379 \\
  --cacert=/etc/kubernetes/pki/etcd/ca.crt \\
  --cert=/etc/kubernetes/pki/etcd/server.crt \\
  --key=/etc/kubernetes/pki/etcd/server.key \\
  --data-dir=/var/lib/etcd-backup \\
  snapshot restore /tmp/etcd-backup.db
\`\`\`
EOF

# Create directory for Kustomize practice
echo "Creating Kustomize practice environment..."
mkdir -p /Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/kustomize-demo/{base,overlays/{staging,production}}

# Create base kustomization files
cat <<EOF > /Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/kustomize-demo/base/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment
  labels:
    app: nginx
spec:
  replicas: 2
  selector:
    matchLabels:
      app: nginx
  template:
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:1.20.0
        ports:
        - containerPort: 80
EOF

cat <<EOF > /Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/kustomize-demo/base/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: nginx-service
spec:
  selector:
    app: nginx
  ports:
  - port: 80
    targetPort: 80
EOF

cat <<EOF > /Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/kustomize-demo/base/kustomization.yaml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
resources:
- deployment.yaml
- service.yaml
EOF

# Create staging overlay
cat <<EOF > /Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/kustomize-demo/overlays/staging/kustomization.yaml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
bases:
- ../../base
namespace: staging
namePrefix: staging-
patchesStrategicMerge:
- deployment-patch.yaml
EOF

cat <<EOF > /Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/kustomize-demo/overlays/staging/deployment-patch.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment
spec:
  replicas: 1
EOF

# Create production overlay
cat <<EOF > /Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/kustomize-demo/overlays/production/kustomization.yaml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
bases:
- ../../base
namespace: production
namePrefix: production-
patchesStrategicMerge:
- deployment-patch.yaml
EOF

cat <<EOF > /Users/simonbalazs/CKA_LAB/05_Cluster_Architecture/kustomize-demo/overlays/production/deployment-patch.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx-deployment
spec:
  replicas: 5
  template:
    spec:
      containers:
      - name: nginx
        resources:
          limits:
            memory: 512Mi
            cpu: 500m
          requests:
            memory: 256Mi
            cpu: 250m
EOF

echo "Cluster Architecture lab environment setup complete!"
echo ""
echo "Available resources:"
echo "- Namespace: rbac-test with service accounts: view-user, edit-user, admin-user"
echo "- Custom Resource Definition: backups.stable.example.com"
echo "- Custom Resource instance: database-backup in rbac-test namespace"
echo "- RBAC Role: pod-reader in rbac-test namespace bound to view-user"
echo "- Deployment: nginx-deploy in rbac-test namespace"
echo "- Kustomize practice files in 05_Cluster_Architecture/kustomize-demo/"
echo "- Kubeadm practice guide in 05_Cluster_Architecture/kubeadm_practice_guide.md"
echo ""
echo "You can now proceed with the Cluster Architecture tasks."
echo "To verify the RBAC setup, try: kubectl auth can-i get pods --as=system:serviceaccount:rbac-test:view-user -n rbac-test"

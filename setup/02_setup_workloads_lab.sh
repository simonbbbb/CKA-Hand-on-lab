#!/bin/bash
# CKA Lab - Workloads and Scheduling Environment Setup Script

set -e

echo "Setting up Workloads and Scheduling Lab Environment..."

# Check if minikube is running
if ! minikube status | grep -q "Running"; then
  echo "Minikube is not running. Starting minikube..."
  minikube start --memory=4096 --cpus=2 --driver=qemu2
fi

# Create a test namespace
echo "Creating test namespace..."
kubectl create namespace workloads-test

# Label nodes for affinity exercises
echo "Labeling node for affinity exercises..."
kubectl label nodes $(kubectl get nodes -o jsonpath='{.items[0].metadata.name}') disk=ssd app=web-server

# Create a simple deployment for practice
echo "Creating sample deployment..."
cat <<EOF | kubectl apply -f -
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
        image: nginx:1.14.2
        ports:
        - containerPort: 80
        resources:
          requests:
            memory: "64Mi"
            cpu: "100m"
          limits:
            memory: "128Mi"
            cpu: "200m"
EOF

# Create ConfigMap and Secret for configuration exercises
echo "Creating ConfigMap and Secret for configuration exercises..."
cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: ConfigMap
metadata:
  name: app-config
data:
  DATABASE_URL: mysql://db:3306/mydb
  DEBUG_MODE: "true"
  CONFIG_FILE: |
    # Sample configuration
    log_level = INFO
    max_connections = 100
    timeout = 30s
EOF

cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: Secret
metadata:
  name: app-secret
type: Opaque
data:
  username: YWRtaW4=
  password: c3VwZXJzZWNyZXQ=
EOF

# Apply taints to practice tolerations
echo "Applying taints to node for toleration exercises..."
kubectl taint nodes $(kubectl get nodes -o jsonpath='{.items[0].metadata.name}') app=critical:NoSchedule

# Create resources for HPA practice
echo "Setting up resources for HPA exercises..."
kubectl apply -f https://k8s.io/examples/application/php-apache.yaml

# Enable metrics server for HPA to work
echo "Enabling metrics-server addon..."
minikube addons enable metrics-server

echo "Workloads and Scheduling lab environment setup complete!"
echo ""
echo "Available resources:"
echo "- Namespace: workloads-test"
echo "- Deployment: nginx-deployment in default namespace"
echo "- ConfigMap: app-config in default namespace"
echo "- Secret: app-secret in default namespace"
echo "- Node with label disk=ssd and taint app=critical:NoSchedule"
echo "- php-apache deployment for HPA practice"
echo "- Metrics Server enabled"
echo ""
echo "You can now proceed with the Workloads and Scheduling tasks."
echo "To verify the setup, run: kubectl get deployments,cm,secrets"

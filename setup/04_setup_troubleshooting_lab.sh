#!/bin/bash
# CKA Lab - Troubleshooting Environment Setup Script

set -e

echo "Setting up Troubleshooting Lab Environment..."

# Check if minikube is running
if ! minikube status | grep -q "Running"; then
  echo "Minikube is not running. Starting minikube..."
  minikube start --memory=4096 --cpus=2 --driver=qemu2
fi

# Create namespace for troubleshooting exercises
echo "Creating troubleshooting namespace..."
kubectl create namespace troubleshooting

# Scenario 1: Problem with a broken pod
echo "Setting up Scenario 1: Pod with image pull error..."
cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: Pod
metadata:
  name: broken-pod
  namespace: troubleshooting
spec:
  containers:
  - name: nginx
    image: nginx:nonexistentversion
    ports:
    - containerPort: 80
EOF

# Scenario 2: Pod with resource constraints
echo "Setting up Scenario 2: Pod with resource constraints..."
cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: Pod
metadata:
  name: resource-constrained-pod
  namespace: troubleshooting
spec:
  containers:
  - name: memory-hog
    image: nginx
    command: ["sh", "-c", "while true; do dd if=/dev/zero of=/dev/null bs=10M count=10; done"]
    resources:
      requests:
        memory: "50Mi"
        cpu: "100m"
      limits:
        memory: "50Mi"
        cpu: "100m"
EOF

# Scenario 3: Pod with configuration issues
echo "Setting up Scenario 3: Pod with configuration issues..."
cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: Pod
metadata:
  name: config-error-pod
  namespace: troubleshooting
spec:
  containers:
  - name: nginx
    image: nginx
    ports:
    - containerPort: 80
    volumeMounts:
    - name: config-volume
      mountPath: /etc/nginx/conf.d
  volumes:
  - name: config-volume
    configMap:
      name: nonexistent-config
EOF

# Scenario 4: Service with selector issues
echo "Setting up Scenario 4: Service with selector issues..."
cat <<EOF | kubectl apply -f -
apiVersion: apps/v1
kind: Deployment
metadata:
  name: web-deployment
  namespace: troubleshooting
spec:
  replicas: 2
  selector:
    matchLabels:
      app: web
  template:
    metadata:
      labels:
        app: web
    spec:
      containers:
      - name: nginx
        image: nginx
        ports:
        - containerPort: 80
---
apiVersion: v1
kind: Service
metadata:
  name: web-service
  namespace: troubleshooting
spec:
  selector:
    app: web-different  # Mismatched selector
  ports:
  - port: 80
    targetPort: 80
EOF

# Scenario 5: Deployment with update issues
echo "Setting up Scenario 5: Deployment with update issues..."
cat <<EOF | kubectl apply -f -
apiVersion: apps/v1
kind: Deployment
metadata:
  name: update-issue-deployment
  namespace: troubleshooting
spec:
  replicas: 3
  selector:
    matchLabels:
      app: update-app
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 1
  template:
    metadata:
      labels:
        app: update-app
    spec:
      containers:
      - name: nginx
        image: nginx:1.14.2
        ports:
        - containerPort: 80
        readinessProbe:
          httpGet:
            path: /nonexistent
            port: 80
          initialDelaySeconds: 10
          periodSeconds: 5
EOF

# Scenario 6: Node level issue (requires admin access, this is more for reference)
echo "Setting up Scenario 6: Instructions for node-level issues..."
cat <<EOF > /Users/simonbalazs/CKA_LAB/04_Troubleshooting/solutions/node_troubleshooting.md
# Node Troubleshooting Guide

Since Minikube runs in a VM, we can't directly simulate node-level issues.
However, in a real-world scenario, you would troubleshoot node issues with:

1. Check node status:
   \`\`\`
   kubectl get nodes
   kubectl describe node <node-name>
   \`\`\`

2. Check kubelet status:
   \`\`\`
   ssh <node>
   systemctl status kubelet
   journalctl -u kubelet
   \`\`\`

3. Check node resources:
   \`\`\`
   ssh <node>
   top
   df -h
   \`\`\`

4. Common node issues:
   - Kubelet not running
   - Certificate issues
   - Network issues
   - Resource exhaustion
   - Container runtime issues

5. Solutions might include:
   - Restarting kubelet: \`systemctl restart kubelet\`
   - Freeing up resources
   - Checking and reconfiguring kubelet
   - Verifying container runtime (Docker/containerd)
EOF

# Scenario 7: Create broken DNS configuration
echo "Setting up Scenario 7: DNS resolution issues..."

# Create pods for DNS testing
cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: Pod
metadata:
  name: dns-client
  namespace: troubleshooting
spec:
  containers:
  - name: dnsutils
    image: registry.k8s.io/e2e-test-images/jessie-dnsutils:1.3
    command:
      - sleep
      - "infinity"
---
apiVersion: v1
kind: Pod
metadata:
  name: dns-target
  namespace: troubleshooting
  labels:
    app: dns-target
spec:
  containers:
  - name: nginx
    image: nginx
---
apiVersion: v1
kind: Service
metadata:
  name: dns-service
  namespace: troubleshooting
spec:
  selector:
    app: dns-target-wrong  # Intentional mismatch
  ports:
  - port: 80
    targetPort: 80
EOF

echo "Troubleshooting lab environment setup complete!"
echo ""
echo "Available troubleshooting scenarios in namespace 'troubleshooting':"
echo "1. Pod with image pull error (broken-pod)"
echo "2. Pod with resource constraints (resource-constrained-pod)"
echo "3. Pod with configuration issues (config-error-pod)"
echo "4. Service with selector issues (web-service and web-deployment)"
echo "5. Deployment with update issues (update-issue-deployment)"
echo "6. Node troubleshooting guide (see solutions/node_troubleshooting.md)"
echo "7. DNS resolution issues (dns-client, dns-target, dns-service)"
echo ""
echo "You can now proceed with the Troubleshooting tasks."
echo "To view the troubled resources, run: kubectl get all -n troubleshooting"

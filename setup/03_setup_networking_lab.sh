#!/bin/bash
# CKA Lab - Servicing and Networking Environment Setup Script

set -e

echo "Setting up Servicing and Networking Lab Environment..."

# Check if minikube is running
if ! minikube status | grep -q "Running"; then
  echo "Minikube is not running. Starting minikube..."
  minikube start --memory=4096 --cpus=2 --driver=qemu2
fi

# Enable Ingress addon
echo "Enabling Ingress addon..."
minikube addons enable ingress

# Create test namespace
echo "Creating test namespace..."
kubectl create namespace networking-test

# Create deployments for service exercises
echo "Creating deployments for service exercises..."

# Create frontend deployment
cat <<EOF | kubectl apply -f -
apiVersion: apps/v1
kind: Deployment
metadata:
  name: frontend
  namespace: networking-test
spec:
  replicas: 3
  selector:
    matchLabels:
      app: frontend
  template:
    metadata:
      labels:
        app: frontend
    spec:
      containers:
      - name: nginx
        image: nginx:latest
        ports:
        - containerPort: 80
        volumeMounts:
        - name: html
          mountPath: /usr/share/nginx/html
      volumes:
      - name: html
        configMap:
          name: frontend-html
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: frontend-html
  namespace: networking-test
data:
  index.html: |
    <!DOCTYPE html>
    <html>
    <head>
      <title>Frontend Service</title>
      <style>
        body { background-color: #f0f8ff; font-family: Arial, sans-serif; text-align: center; padding-top: 50px; }
      </style>
    </head>
    <body>
      <h1>Frontend Service</h1>
      <p>This is the frontend service for the CKA networking lab.</p>
      <p>Try to access the backend service at: <a href="/api">/api</a></p>
    </body>
    </html>
EOF

# Create backend deployment
cat <<EOF | kubectl apply -f -
apiVersion: apps/v1
kind: Deployment
metadata:
  name: backend
  namespace: networking-test
spec:
  replicas: 2
  selector:
    matchLabels:
      app: backend
  template:
    metadata:
      labels:
        app: backend
    spec:
      containers:
      - name: backend
        image: nginx:latest
        ports:
        - containerPort: 80
        volumeMounts:
        - name: html
          mountPath: /usr/share/nginx/html
      volumes:
      - name: html
        configMap:
          name: backend-html
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: backend-html
  namespace: networking-test
data:
  index.html: |
    <!DOCTYPE html>
    <html>
    <head>
      <title>Backend API</title>
      <style>
        body { background-color: #e6ffe6; font-family: Arial, sans-serif; text-align: center; padding-top: 50px; }
      </style>
    </head>
    <body>
      <h1>Backend API Service</h1>
      <p>This is the backend API service for the CKA networking lab.</p>
      <pre>{"status": "ok", "message": "API is working", "version": "1.0"}</pre>
    </body>
    </html>
EOF

# Create database deployment
cat <<EOF | kubectl apply -f -
apiVersion: apps/v1
kind: Deployment
metadata:
  name: db
  namespace: networking-test
spec:
  replicas: 1
  selector:
    matchLabels:
      app: db
  template:
    metadata:
      labels:
        app: db
    spec:
      containers:
      - name: postgres
        image: postgres:12
        ports:
        - containerPort: 5432
        env:
        - name: POSTGRES_PASSWORD
          value: password
        - name: POSTGRES_USER
          value: user
        - name: POSTGRES_DB
          value: testdb
EOF

# Create services
echo "Creating services..."
cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: Service
metadata:
  name: frontend-service
  namespace: networking-test
spec:
  selector:
    app: frontend
  ports:
  - port: 80
    targetPort: 80
  type: ClusterIP
---
apiVersion: v1
kind: Service
metadata:
  name: backend-service
  namespace: networking-test
spec:
  selector:
    app: backend
  ports:
  - port: 80
    targetPort: 80
  type: ClusterIP
---
apiVersion: v1
kind: Service
metadata:
  name: db-service
  namespace: networking-test
spec:
  selector:
    app: db
  ports:
  - port: 5432
    targetPort: 5432
  type: ClusterIP
EOF

# Create NodePort service example
cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: Service
metadata:
  name: frontend-nodeport
  namespace: networking-test
spec:
  selector:
    app: frontend
  ports:
  - port: 80
    targetPort: 80
    nodePort: 30080
  type: NodePort
EOF

# Create ingress resource
echo "Creating Ingress resource..."
cat <<EOF | kubectl apply -f -
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: app-ingress
  namespace: networking-test
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
spec:
  rules:
  - host: myapp.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: frontend-service
            port:
              number: 80
      - path: /api
        pathType: Prefix
        backend:
          service:
            name: backend-service
            port:
              number: 80
EOF

# Create network policy
echo "Creating Network Policy..."
cat <<EOF | kubectl apply -f -
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: db-network-policy
  namespace: networking-test
spec:
  podSelector:
    matchLabels:
      app: db
  policyTypes:
  - Ingress
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: backend
    ports:
    - protocol: TCP
      port: 5432
EOF

echo "Adding entry to /etc/hosts for ingress testing..."
echo "The following line would need to be added to /etc/hosts to test Ingress:"
echo "$(minikube ip) myapp.example.com"
echo "(Note: You may need to run this with sudo privileges)"

echo "Servicing and Networking lab environment setup complete!"
echo ""
echo "Available resources:"
echo "- Namespace: networking-test"
echo "- Deployments: frontend, backend, and db in networking-test namespace"
echo "- Services: frontend-service, backend-service, db-service (ClusterIP)"
echo "- NodePort Service: frontend-nodeport exposed on port 30080"
echo "- Ingress: app-ingress routing traffic for myapp.example.com"
echo "- Network Policy: db-network-policy restricting access to the db"
echo ""
echo "You can now proceed with the Servicing and Networking tasks."
echo "To verify the setup, run: kubectl get all,ingress,networkpolicy -n networking-test"

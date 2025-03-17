# Kubernetes and Helm Cheat Sheet

## Node/System Debugging

### Basic Node Diagnostics
```bash
kubectl get nodes
kubectl describe node <node-name>
```

### Node System Service Management
```bash
# Check kubelet service status
sudo systemctl status kubelet

# View kubelet logs
sudo journalctl -u kubelet -f

# Check other critical services
sudo systemctl status docker
sudo systemctl status containerd

# Service management commands
sudo systemctl start|stop|restart kubelet
sudo systemctl enable kubelet
```

### Log Viewing with journalctl
```bash
# View last 100 lines
sudo journalctl -u kubelet -n 100

# View logs since specific time
sudo journalctl -u kubelet --since "2023-03-17 10:00:00"

# Follow logs in real-time
sudo journalctl -u kubelet -f

# Show logs with priority higher than error
sudo journalctl -u kubelet -p err
```

### Important File Locations
- **/var/log/containers/** - Container logs
- **/var/log/pods/** - Pod logs
- **/var/lib/kubelet/** - Kubelet working directory
- **/etc/kubernetes/** - Kubernetes configuration files
- **/etc/kubernetes/manifests/** - Static pod manifests
- **/var/lib/etcd/** - etcd data directory

### Control Plane Components
```bash
sudo systemctl status kube-apiserver
sudo systemctl status kube-controller-manager
sudo systemctl status kube-scheduler
sudo systemctl status etcd
```

## Resource Debugging

### Pod and Resource Inspection
```bash
# Detailed resource info
kubectl describe pod <pod-name>
kubectl describe deployment <deployment-name>

# Check logs
kubectl logs <pod-name>
kubectl logs <pod-name> -c <container-name>  # Multi-container pods
kubectl logs <pod-name> --previous  # Previous container instance

# Enhanced output formatting
kubectl get pods -o wide  # Shows node allocation
kubectl get pods -o yaml  # Full YAML definition
kubectl get pods -o json | jq  # JSON with filtering

# Resource usage
kubectl top nodes
kubectl top pods
```

### Interactive Debugging
```bash
# Execute commands in containers
kubectl exec -it <pod-name> -- /bin/bash
kubectl exec -it <pod-name> -c <container-name> -- /bin/sh

# Port-forward for testing
kubectl port-forward <pod-name> 8080:80

# Check events
kubectl get events --sort-by='.metadata.creationTimestamp'

# Schema help
kubectl explain pod.spec
kubectl explain deployment.spec.strategy

# Create temporary debugging pods
kubectl run debug --image=busybox --rm -it -- /bin/sh

# Check DNS resolution
kubectl run dns-test --image=busybox --rm -it -- nslookup kubernetes.default
```

### Advanced Debugging

```bash
# Debug with ephemeral containers
kubectl debug pod/<pod-name> -it --image=ubuntu --target=<container-name>

# Check NetworkPolicies
kubectl get networkpolicies --all-namespaces

# RBAC debugging
kubectl auth can-i list pods --namespace=default
kubectl auth can-i create deployments --as=system:serviceaccount:default:default

# Check QoS class
kubectl get pod <pod-name> -o jsonpath='{.status.qosClass}'

# Quick resource patching
kubectl patch deployment <name> -p '{"spec":{"replicas":3}}'

# Check service endpoints
kubectl get endpoints <service-name>

# Use JSONPath for targeted info
kubectl get pods -o jsonpath='{.items[*].metadata.name}'
kubectl get nodes -o jsonpath='{.items[*].status.addresses[?(@.type=="InternalIP")].address}'

# Deployment rollout management
kubectl rollout status deployment/<name>
kubectl rollout history deployment/<name>
kubectl rollout undo deployment/<name>

# Check PodDisruptionBudgets
kubectl get pdb

# Container probe inspection
kubectl get pod <pod-name> -o jsonpath='{.spec.containers[*].livenessProbe}'

# DNS and connectivity testing
kubectl run curl --image=curlimages/curl -it --rm -- curl -v <service-name>.<namespace>.svc.cluster.local

# Node drain testing
kubectl drain <node> --ignore-daemonsets --dry-run
```

## Helm Commands

### Repository Management
```bash
# Add a repository
helm repo add [name] [url]
helm repo add stable https://charts.helm.sh/stable

# Update repositories
helm repo update

# Search for charts
helm search repo [keyword]
helm search hub [keyword]

# View chart information
helm show chart [repo/chart]
helm show values [repo/chart]
helm show all [repo/chart]
```

### Installing and Managing Charts
```bash
# Install a chart
helm install [release-name] [chart] --namespace [namespace] --create-namespace
helm install nginx bitnami/nginx --namespace web --create-namespace

# Install with custom values
helm install [release-name] [chart] -f values.yaml
helm install [release-name] [chart] --set key1=value1,key2=value2

# List releases
helm list --all-namespaces
helm list -n [namespace]

# Get release status
helm status [release-name] -n [namespace]

# Upgrade a release
helm upgrade [release-name] [chart] -n [namespace]
helm upgrade --install [release-name] [chart]  # Install if not exists

# Rollback to previous version
helm rollback [release-name] [revision] -n [namespace]

# Uninstall a release
helm uninstall [release-name] -n [namespace]
```

### Chart Development
```bash
# Create a new chart
helm create [chart-name]

# Lint a chart
helm lint [chart-directory]

# Package a chart
helm package [chart-directory]

# Install from local directory
helm install [release-name] ./[chart-directory]

# Template rendering (dry run)
helm template [release-name] [chart] --namespace [namespace]

# Test a release
helm test [release-name] -n [namespace]
```

### Advanced Helm Usage
```bash
# Debug
helm install --debug --dry-run [release-name] [chart]

# Get manifest content
helm get manifest [release-name] -n [namespace]

# Get values used
helm get values [release-name] -n [namespace]

# Get all historical revisions
helm history [release-name] -n [namespace]

# Pull chart to local directory
helm pull [repo/chart] --untar

# Diff between values/revisions (requires helm-diff plugin)
helm diff upgrade [release-name] [chart] -f values.yaml
helm diff revision [release-name] [rev1] [rev2]
```

### Useful Helm Plugins
```bash
# Install plugins
helm plugin install https://github.com/databus23/helm-diff
helm plugin install https://github.com/chartmuseum/helm-push
helm plugin install https://github.com/kraft-aka-j/helm-find

# Plugin usage examples
helm diff upgrade [release] [chart]
helm push [chart] [repo]
helm find [keyword]
```

## CKA Exam Tips

1. Use shell aliases to save time: `alias k=kubectl` and `alias h=helm`
2. Use kubectl autocomplete: `source <(kubectl completion bash)`
3. Use `kubectl explain` instead of looking up documentation
4. Use `--dry-run=client -o yaml` to generate manifests quickly
5. Use `watch kubectl get pods` to monitor resource changes
6. Always specify namespace with `-n` flag
7. Check events when troubleshooting: `kubectl get events --sort-by=.metadata.creationTimestamp`
8. Use CTRL+R to search bash history for previous commands
9. Keep track of your context with `kubectl config current-context`
10. Skip difficult questions and come back to them later
11. Create your own .vimrc file for better editor experience during the exam
12. Use imperative commands over declarative when possible for speed
13. Memorize the syntax for the most common resources (Pods, Deployments, Services)
14. Learn to use JSONPath effectively for output filtering
15. Use CLI bookmarks to save common command patterns
16. Become familiar with the official K8s documentation structure before the exam
17. Practice using vim/nano efficiently before the exam
18. Keep a mental checklist of verification steps for each task
19. Use kubectl explain recursively (e.g., `kubectl explain pod.spec.containers.resources`)
20. Set up a systematic troubleshooting approach for each resource type

## Quick Reference: One-Liners for Common Tasks

### Resource Creation and Management
```bash
# Create a deployment and scale it
kubectl create deployment nginx --image=nginx
kubectl scale deployment nginx --replicas=3

# Create a ConfigMap and Secret from literals
kubectl create configmap app-config --from-literal=APP_ENV=production
kubectl create secret generic db-creds --from-literal=username=admin --from-literal=password=secret

# Create a job and cronjob
kubectl create job test-job --image=busybox -- /bin/sh -c "echo hello"
kubectl create cronjob test-cron --image=busybox --schedule="*/5 * * * *" -- /bin/sh -c "echo hello"

# Create a pod with specific resource limits
kubectl run nginx --image=nginx --requests=cpu=100m,memory=128Mi --limits=cpu=200m,memory=256Mi

# Generate service YAML for a deployment
kubectl expose deployment nginx --port=80 --target-port=8080 --dry-run=client -o yaml

# Quickly create a PV and PVC
kubectl create pv mypv --capacity=1Gi --access-modes=ReadWriteOnce --path=/tmp/data
kubectl create pvc mypvc --access-modes=ReadWriteOnce --storage=1Gi
```

### Advanced Networking Commands
```bash
# Check if a port is listening on a pod
kubectl exec -it <pod-name> -- netstat -tuln

# Test network connectivity from a pod
kubectl exec -it <pod-name> -- wget -O- <service-name>:<port>

# Check DNS resolution chain
kubectl run dns-test --image=tutum/dnsutils --restart=Never --rm -it -- dig +search <service-name>

# Get all services and their cluster IPs
kubectl get svc -A -o custom-columns="NAMESPACE:.metadata.namespace,NAME:.metadata.name,CLUSTER-IP:.spec.clusterIP"

# Identify service endpoint connections
kubectl get endpoints <service-name> -o jsonpath='{.subsets[0].addresses[*].ip}'
```

### ETCD Operations
```bash
# Backup etcd
ETCDCTL_API=3 etcdctl --endpoints=https://127.0.0.1:2379 \
  --cacert=/etc/kubernetes/pki/etcd/ca.crt \
  --cert=/etc/kubernetes/pki/etcd/server.crt \
  --key=/etc/kubernetes/pki/etcd/server.key \
  snapshot save /tmp/etcd-backup.db

# Restore etcd from backup
ETCDCTL_API=3 etcdctl snapshot restore /tmp/etcd-backup.db \
  --data-dir=/var/lib/etcd-restore

# Check etcd health
ETCDCTL_API=3 etcdctl --endpoints=https://127.0.0.1:2379 \
  --cacert=/etc/kubernetes/pki/etcd/ca.crt \
  --cert=/etc/kubernetes/pki/etcd/server.crt \
  --key=/etc/kubernetes/pki/etcd/server.key \
  endpoint health

# List all keys in etcd
ETCDCTL_API=3 etcdctl --endpoints=https://127.0.0.1:2379 \
  --cacert=/etc/kubernetes/pki/etcd/ca.crt \
  --cert=/etc/kubernetes/pki/etcd/server.crt \
  --key=/etc/kubernetes/pki/etcd/server.key \
  get / --prefix --keys-only
```

### Certificate Management
```bash
# Check certificate expiration
kubeadm certs check-expiration

# Generate certificate signing request
openssl req -new -key server.key -out server.csr -subj "/CN=example.com/O=system:masters"

# Approve CSR
kubectl certificate approve <csr-name>

# Inspect certificate contents
openssl x509 -in /etc/kubernetes/pki/apiserver.crt -text -noout

# View certificate signing request details
kubectl get csr <csr-name> -o jsonpath='{.spec.request}' | base64 --decode | openssl req -noout -text
```

### Efficient RBAC Management
```bash
# Create role with multiple rules quickly
kubectl create role developer --verb=get,list,watch --resource=pods,deployments,services -n development

# Bind role to user
kubectl create rolebinding dev-user-binding --role=developer --user=jane -n development

# Check if user has specific permissions
kubectl auth can-i list pods --as=jane -n development

# Create a service account and give it cluster-admin
kubectl create sa admin-user
kubectl create clusterrolebinding admin-user-binding --clusterrole=cluster-admin --serviceaccount=default:admin-user

# Get all roles in a namespace with their rules
kubectl get roles -n default -o jsonpath='{range .items[*]}{.metadata.name}{"\n"}{range .rules[*]}{.verbs}{" "}{.resources}{"\n"}{end}{"\n"}{end}'
```

### Kubelet Troubleshooting
```bash
# Check kubelet configuration
sudo cat /var/lib/kubelet/config.yaml

# Review kubelet command line arguments
ps aux | grep kubelet

# Check certificate paths used by kubelet
grep -r "pki" /etc/systemd/system/kubelet.service.d/

# Verify node conditions
kubectl get node <node-name> -o jsonpath='{.status.conditions[*].type}{"\n"}{.status.conditions[*].status}'

# Check container runtime status
sudo crictl info
sudo crictl ps
```

### CKA-Specific Simulation Tasks
```bash
# Drain a node properly
kubectl drain <node-name> --ignore-daemonsets --delete-emptydir-data

# Join a node to cluster (reuse existing token)
kubeadm token list
kubeadm join <endpoint>:<port> --token <token> --discovery-token-ca-cert-hash sha256:<hash>

# Create new join token
kubeadm token create --print-join-command

# Update kubeadm init configuration
cat > kubeadm-config.yaml << EOF
apiVersion: kubeadm.k8s.io/v1beta3
kind: InitConfiguration
nodeRegistration:
  name: "$(hostname -s)"
---
apiVersion: kubeadm.k8s.io/v1beta3
kind: ClusterConfiguration
kubernetesVersion: v1.27.0
networking:
  podSubnet: "10.244.0.0/16"
  serviceSubnet: "10.96.0.0/12"
EOF

# Configure high availability
sudo kubeadm init --config=kubeadm-ha-config.yaml --upload-certs
```

## Exam Preparation Strategies

1. **Time-boxed Practice**: Set a timer for 2 hours and try to solve as many lab tasks as possible
2. **Simulated Exam Environment**: Practice without internet access except for kubernetes.io docs
3. **Command Reflex Building**: Practice typing common commands until they become muscle memory
4. **Troubleshooting Drills**: Deliberately break clusters in specific ways and fix them
5. **Documentation Navigation**: Practice finding specific topics in kubernetes.io documentation
6. **Keyboard Shortcut Mastery**: Learn bash/vim shortcuts to increase efficiency
7. **Imperative Command Fluency**: Build confidence with kubectl create and kubectl run variations
8. **Question Triage**: Practice skipping harder questions to manage time effectively
9. **YAML Structure Memorization**: Know the basic structure of key resources by heart
10. **Mental Checklists**: Create verification checklists for each type of task
11. **Daily Practice**: Spend at least 30 minutes daily on practical scenarios
12. **Resource Allocation**: Know memory/CPU units and how to specify them quickly
13. **Contextual Awareness**: Practice switching between clusters and namespaces efficiently
14. **Error Recognition**: Learn to quickly interpret common error messages
15. **Command Output Analysis**: Practice extracting key info from command output
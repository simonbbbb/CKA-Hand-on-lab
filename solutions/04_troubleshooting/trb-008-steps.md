# TRB-008: Fix kube-apiserver Static Pod - Step-by-Step

## Problem
The kube-apiserver static pod is not running. The API server cannot connect to etcd
because the `--etcd-servers` flag points to the wrong URL.

## Diagnosis Steps

### 1. Attempt to reach the API server
```bash
kubectl get nodes
# Expected: connection refused or timeout
```

### 2. SSH into the control plane node
```bash
ssh controlplane
```

### 3. Check which static pod containers are running
```bash
crictl ps
# Note: kube-apiserver container is missing
```

### 4. Check kubelet logs for API server errors
```bash
journalctl -u kubelet | grep apiserver | tail -20
# Look for errors about the static pod failing to start
```

### 5. Inspect the static pod manifest
```bash
sudo cat /etc/kubernetes/manifests/kube-apiserver.yaml | grep etcd-servers
# The output shows: - --etcd-servers=http://192.168.99.50:2379
# This is wrong -- it should be https://127.0.0.1:2379
```

## Fix Steps

### 6. Edit the static pod manifest
```bash
sudo vi /etc/kubernetes/manifests/kube-apiserver.yaml
```

Find the line:
```yaml
    - --etcd-servers=http://192.168.99.50:2379
```

Replace with:
```yaml
    - --etcd-servers=https://127.0.0.1:2379
```

Save and exit (`:wq` in vim).

### 7. Wait for the kubelet to detect the change
The kubelet watches `/etc/kubernetes/manifests/` and will automatically detect the
change and restart the API server pod. This typically takes 20-60 seconds.

```bash
# Watch for the API server container to appear
watch crictl ps
```

### 8. Verify the API server is running
```bash
crictl ps | grep kube-apiserver
# Should show a running container
```

### 9. Exit SSH and verify cluster access
```bash
exit  # Exit the SSH session
kubectl get nodes
# The control plane node should show status Ready
```

## Key Takeaways
- Static pods are defined in YAML files under `/etc/kubernetes/manifests/`.
- The kubelet automatically manages static pods based on these manifest files.
- Common API server misconfigurations include wrong etcd URLs, incorrect certificate
  paths, and wrong bind addresses.
- When the API server is down, you must use node-level tools (crictl, journalctl)
  instead of kubectl.

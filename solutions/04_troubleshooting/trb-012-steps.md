# TRB-012: CNI Plugin Fix - Step-by-Step

## Problem
Pods on different nodes cannot communicate. The CNI plugin pod on worker01 is
CrashLooping due to an incorrect network CIDR configuration.

## Diagnosis Steps

### 1. Check CNI pod status
```bash
kubectl get pods -n kube-system -o wide
# Identify the CNI pod (e.g., weave-net, calico-node, kube-flannel) that is
# CrashLoopBackOff on worker01.
```

### 2. Get pod IPs of test pods
```bash
kubectl get pods -o wide
# Note the IPs of test-pod-a (on controlplane) and test-pod-b (on worker01)
```

### 3. Test cross-node connectivity (confirm the problem)
```bash
kubectl exec test-pod-a -- ping -c 3 <test-pod-b-ip>
# Expected: 100% packet loss
```

### 4. Check the CrashLooping CNI pod logs
```bash
kubectl logs <cni-pod-name> -n kube-system
# Look for errors about network range mismatch, configuration errors, or
# "incompatible CIDR" messages.
```

### 5. Check the cluster's expected pod CIDR
```bash
# Option A: Check kubeadm config
kubectl get cm kubeadm-config -n kube-system -o yaml | grep podSubnet

# Option B: Check controller manager static pod
cat /etc/kubernetes/manifests/kube-controller-manager.yaml | grep cluster-cidr
# Expected: --cluster-cidr=10.244.0.0/16
```

### 6. SSH into the failing node
```bash
ssh worker01
```

### 7. Inspect CNI configuration
```bash
sudo cat /etc/cni/net.d/10-flannel.conflist
# Or for weave:
#   sudo cat /etc/cni/net.d/10-weave.conflist
# Or for calico:
#   sudo cat /etc/cni/net.d/10-calico.conflist
```

### 8. Check the Flannel ConfigMap for wrong CIDR
```bash
kubectl get configmap kube-flannel-cfg -n kube-flannel -o yaml | grep -A5 net-conf
# The "Network" field shows "10.32.0.0/12" -- this is wrong.
```

## Fix Steps

### 9. Fix the CNI ConfigMap
```bash
kubectl edit configmap kube-flannel-cfg -n kube-flannel
```

Find the `net-conf.json` section and change:
```json
"Network": "10.32.0.0/12"
```

To:
```json
"Network": "10.244.0.0/16"
```

Save and exit.

### 10. Fix the CNI config file on the node
```bash
# Still on worker01:
sudo vi /etc/cni/net.d/10-flannel.conflist
# Ensure the network configuration matches the cluster CIDR.
```

### 11. Restart the CNI pod
```bash
# Exit SSH first
exit

# Delete the CrashLooping pod (DaemonSet will recreate it)
kubectl delete pod <cni-pod-name> -n kube-system
```

### 12. Wait for the new CNI pod to start
```bash
kubectl get pods -n kube-system -o wide -w
# Wait until the CNI pod on worker01 is Running
```

### 13. Re-test cross-node connectivity
```bash
kubectl exec test-pod-a -- ping -c 3 <test-pod-b-ip>
# Expected: successful ping with 0% packet loss
```

### 14. Verify the CNI pod is healthy
```bash
kubectl logs <new-cni-pod-name> -n kube-system
# Should show normal startup messages without errors
```

## Alternative: Quick Reinstall of CNI
If fixing the config does not work, a fast alternative is to reinstall the CNI plugin:
```bash
# For Flannel:
kubectl delete -f https://raw.githubusercontent.com/flannel-io/flannel/master/Documentation/kube-flannel.yml
kubectl apply -f https://raw.githubusercontent.com/flannel-io/flannel/master/Documentation/kube-flannel.yml

# For Weave:
kubectl delete -f "https://cloud.weave.works/k8s/net.yaml?k8s-version=$(kubectl version | base64 | tr -d '\n')"
kubectl apply -f "https://cloud.weave.works/k8s/net.yaml?k8s-version=$(kubectl version | base64 | tr -d '\n')"
```

## Key Takeaways
- CNI issues affect cross-node pod communication. Same-node communication may still work
  because it uses the local bridge.
- The pod network CIDR must be consistent across: the cluster configuration
  (--cluster-cidr), the CNI ConfigMap, and the CNI config files on each node.
- CNI plugins run as DaemonSets -- deleting the pod causes the DaemonSet to recreate it.
- In the exam, if the CNI fix is taking too long, consider reinstalling the CNI plugin
  entirely as a faster alternative.

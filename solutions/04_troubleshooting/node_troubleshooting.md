# Node Troubleshooting Guide

Since Minikube runs in a VM, we can't directly simulate node-level issues.
However, in a real-world scenario, you would troubleshoot node issues with:

1. Check node status:
   ```
   kubectl get nodes
   kubectl describe node <node-name>
   ```

2. Check kubelet status:
   ```
   ssh <node>
   systemctl status kubelet
   journalctl -u kubelet
   ```

3. Check node resources:
   ```
   ssh <node>
   top
   df -h
   ```

4. Common node issues:
   - Kubelet not running
   - Certificate issues
   - Network issues
   - Resource exhaustion
   - Container runtime issues

5. Solutions might include:
   - Restarting kubelet: `systemctl restart kubelet`
   - Freeing up resources
   - Checking and reconfiguring kubelet
   - Verifying container runtime (Docker/containerd)

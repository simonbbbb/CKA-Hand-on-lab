# TRB-009: Kubelet Certificate Expiry Fix - Step-by-Step

## Problem
Worker node `worker01` is NotReady because the kubelet's client certificate has expired.
The kubelet cannot authenticate to the API server.

## Diagnosis Steps

### 1. Check node status
```bash
kubectl get nodes
# worker01 shows NotReady
```

### 2. Get detailed node information
```bash
kubectl describe node worker01
# Check the Conditions section:
#   Ready   Unknown   ...
# Messages may indicate kubelet is not posting status
```

### 3. SSH into the affected node
```bash
ssh worker01
```

### 4. Check kubelet logs
```bash
sudo journalctl -u kubelet -n 50 --no-pager
# Look for errors like:
#   "x509: certificate has expired or is not yet valid"
#   "failed to rotate client certificates"
#   "Unable to authenticate" with certificate-related messages
```

### 5. Check certificate expiration
```bash
sudo kubeadm certs check-expiration
# Output shows which certificates have expired or are about to expire:
#   CERTIFICATE                EXPIRES
#   apiserver                  ... (expired)
#   apiserver-etcd-client      ... (expired)
#   ...
```

## Fix Steps

### 6. Renew all expired certificates
```bash
sudo kubeadm certs renew all
# Output confirms each certificate was renewed:
#   Certificate for serving the Kubernetes API renewed.
#   Certificate for the API server to connect to etcd renewed.
#   ...
```

### 7. Restart the kubelet service
```bash
sudo systemctl restart kubelet
```

### 8. Verify kubelet is running
```bash
sudo systemctl status kubelet
# Should show active (running)
```

### 9. Exit SSH and verify the node is Ready
```bash
exit  # Exit the SSH session
kubectl get nodes
# worker01 should now show status Ready
```

### 10. Confirm with a detailed check
```bash
kubectl get node worker01 -o jsonpath='{.status.conditions[?(@.type=="Ready")].status}'
# Output: True
```

## Additional Notes

If certificate renewal also affects the admin.conf on the control plane:
```bash
# On the control plane node:
sudo cp /etc/kubernetes/admin.conf ~/.kube/config
# This ensures kubectl uses the renewed certificates
```

To check individual certificate expiration at any time:
```bash
sudo openssl x509 -in /var/lib/kubelet/pki/kubelet-client-current.pem \
  -noout -text | grep "Not Before\|Not After"
```

## Key Takeaways
- Kubelet certificates are managed by kubeadm and have a default validity of 1 year.
- The `kubeadm certs renew all` command renews all kubeadm-managed certificates.
- Always restart kubelet after certificate renewal: `systemctl restart kubelet`.
- The `kubeadm certs check-expiration` command is the quickest way to identify
  expired or soon-to-expire certificates.

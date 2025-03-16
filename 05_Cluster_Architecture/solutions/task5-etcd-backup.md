# ETCD Backup and Restore Procedure

## Backup ETCD

### 1. Identify ETCD Pod/Container
```bash
# In a kubeadm setup
kubectl -n kube-system get pods | grep etcd

# Or find the ETCD endpoint
kubectl -n kube-system describe pod etcd-master | grep listen-client-urls
```

### 2. Create the Backup
```bash
# Using etcdctl directly on the master node
ETCDCTL_API=3 etcdctl --endpoints=https://127.0.0.1:2379 \
  --cacert=/etc/kubernetes/pki/etcd/ca.crt \
  --cert=/etc/kubernetes/pki/etcd/server.crt \
  --key=/etc/kubernetes/pki/etcd/server.key \
  snapshot save /opt/etcd-backup.db
```

## Restore ETCD from Backup

### 1. Stop the Kubernetes API Server
```bash
# On the master node
sudo systemctl stop kubelet
sudo docker stop $(docker ps -q --filter name=k8s_kube-apiserver*)
```

### 2. Restore the Snapshot
```bash
# Create a new data directory
sudo mkdir -p /var/lib/etcd-backup

# Restore the snapshot
ETCDCTL_API=3 etcdctl --endpoints=https://127.0.0.1:2379 \
  --cacert=/etc/kubernetes/pki/etcd/ca.crt \
  --cert=/etc/kubernetes/pki/etcd/server.crt \
  --key=/etc/kubernetes/pki/etcd/server.key \
  snapshot restore /opt/etcd-backup.db \
  --data-dir=/var/lib/etcd-backup \
  --name=master \
  --initial-cluster=master=https://127.0.0.1:2380 \
  --initial-cluster-token=etcd-cluster-1 \
  --initial-advertise-peer-urls=https://127.0.0.1:2380
```

### 3. Update ETCD Configuration
```bash
# Edit the ETCD static pod manifest
sudo vim /etc/kubernetes/manifests/etcd.yaml

# Update the volumes section to point to the new data directory
# Change:
#   - hostPath:
#       path: /var/lib/etcd
#       type: DirectoryOrCreate
#     name: etcd-data
# To:
#   - hostPath:
#       path: /var/lib/etcd-backup
#       type: DirectoryOrCreate
#     name: etcd-data
```

### 4. Restart Kubernetes Services
```bash
# Restart the kubelet service
sudo systemctl start kubelet

# Verify the cluster is functioning
kubectl get nodes
kubectl get pods --all-namespaces
```

## Important Notes
1. Always test the backup and restore procedure in a non-production environment first
2. Keep multiple backups with proper timestamps
3. Store backups in a secure, offsite location
4. Document the procedure specific to your environment
5. Regular backups should be scheduled as part of maintenance

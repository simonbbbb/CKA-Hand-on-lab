# Storage (10%)

This section covers implementing storage solutions in Kubernetes.

## Key Concepts

- Storage Classes
- Persistent Volumes (PV)
- Persistent Volume Claims (PVC)
- Volume Access Modes
- Reclaim Policies
- Dynamic Volume Provisioning

## Practice Questions

1. What is the default reclaim policy for dynamically provisioned persistent volumes?
2. Explain the difference between the ReadWriteOnce, ReadOnlyMany, and ReadWriteMany access modes.
3. How would you change the reclaim policy of an existing PersistentVolume from Delete to Retain?
4. What happens to a PersistentVolume when its PersistentVolumeClaim is deleted and the reclaim policy is set to "Retain"?
5. What is the purpose of a StorageClass in Kubernetes?

## Hands-on Tasks

### Task 1: Create a StorageClass

Create a StorageClass named `fast-storage` with the following specifications:
- Provisioner: `kubernetes.io/no-provisioner`
- Volume Binding Mode: `WaitForFirstConsumer`
- Reclaim Policy: `Delete`

### Task 2: Create a Persistent Volume

Create a Persistent Volume with the following specifications:
- Name: `pv-manual`
- Storage: 1Gi
- Access Mode: ReadWriteOnce
- Host Path: `/mnt/data`
- StorageClass: `manual`

### Task 3: Create a Persistent Volume Claim

Create a Persistent Volume Claim that binds to the PV created in Task 2:
- Name: `pvc-manual`
- Storage Request: 500Mi
- Access Mode: ReadWriteOnce
- StorageClass: `manual`

### Task 4: Create a Pod that uses the PVC

Create a Pod that uses the PVC created in Task 3:
- Name: `pod-with-pvc`
- Image: `nginx`
- Mount the PVC at `/usr/share/nginx/html`

### Task 5: Implement Dynamic Volume Provisioning

Create a StorageClass that supports dynamic provisioning (if your environment supports it), create a PVC that uses this StorageClass, and create a Pod that uses this PVC.

## Solutions

The solutions for these tasks can be found in the [solutions](./solutions/) directory.

## Additional Resources

- [Kubernetes Storage Documentation](https://kubernetes.io/docs/concepts/storage/)
- [Persistent Volumes](https://kubernetes.io/docs/concepts/storage/persistent-volumes/)
- [Storage Classes](https://kubernetes.io/docs/concepts/storage/storage-classes/)

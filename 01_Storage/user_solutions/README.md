# Storage User Solutions

Place your storage solution YAML files in this directory. This directory is gitignored, so your solutions won't be committed to the repository.

## Recommended File Naming

For each task, create a separate YAML file with a descriptive name:

- Task 1: `01-fast-storage.yaml` (StorageClass)
- Task 2: `02-pv-manual.yaml` (PersistentVolume)
- Task 3: `03-pvc-manual.yaml` (PersistentVolumeClaim)
- Task 4: `04-pod-with-pvc.yaml` (Pod using PVC)
- Task 5: `05-dynamic-provisioning.yaml` (DynamicProvisioning)

## Applying Your Solutions

Apply your solutions to the Kubernetes cluster:

```bash
kubectl apply -f 01-fast-storage.yaml
kubectl apply -f 02-pv-manual.yaml
# ...and so on
```

## Verifying Your Solutions

After applying your solutions, verify them using the verification script:

```bash
cd ../../setup
./verify_solutions.sh
# Then select option 1 for Storage
```

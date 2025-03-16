# Workloads and Scheduling User Solutions

Place your workloads and scheduling solution YAML files in this directory. This directory is gitignored, so your solutions won't be committed to the repository.

## Recommended File Naming

For each task, create a separate YAML file with a descriptive name:

- Task 1: `01-deployment.yaml` 
- Task 2: `02-pod-affinity.yaml`
- Task 3: `03-configmap-secret.yaml`
- Task 4: `04-hpa.yaml`
- Task 5: `05-daemon-set.yaml`

## Applying Your Solutions

Apply your solutions to the Kubernetes cluster:

```bash
kubectl apply -f 01-deployment.yaml
kubectl apply -f 02-pod-affinity.yaml
# ...and so on
```

## Verifying Your Solutions

After applying your solutions, verify them using the verification script:

```bash
cd ../../setup
./verify_solutions.sh
# Then select option 2 for Workloads and Scheduling
```

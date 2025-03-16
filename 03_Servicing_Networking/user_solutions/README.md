# Networking User Solutions

Place your networking solution YAML files in this directory. This directory is gitignored, so your solutions won't be committed to the repository.

## Recommended File Naming

For each task, create a separate YAML file with a descriptive name:

- Task 1: `01-service.yaml` 
- Task 2: `02-ingress.yaml`
- Task 3: `03-network-policy.yaml`
- Task 4: `04-service-dns.yaml`
- Task 5: `05-nodeport-service.yaml`

## Applying Your Solutions

Apply your solutions to the Kubernetes cluster:

```bash
kubectl apply -f 01-service.yaml
kubectl apply -f 02-ingress.yaml
# ...and so on
```

## Verifying Your Solutions

After applying your solutions, verify them using the verification script:

```bash
cd ../../setup
./verify_solutions.sh
# Then select option 3 for Networking
```

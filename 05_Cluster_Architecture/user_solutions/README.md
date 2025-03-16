# Cluster Architecture User Solutions

Place your cluster architecture solution YAML files in this directory. This directory is gitignored, so your solutions won't be committed to the repository.

## Recommended File Naming

For each task, create a separate YAML file with a descriptive name:

- Task 1: `01-rbac-role.yaml` 
- Task 2: `02-rbac-rolebinding.yaml`
- Task 3: `03-crd.yaml`
- Task 4: `04-kubeadm-config.yaml`
- Task 5: `05-etcd-backup.md` (Commands and procedures)

## Applying Your Solutions

Apply your solutions to the Kubernetes cluster:

```bash
kubectl apply -f 01-rbac-role.yaml
kubectl apply -f 02-rbac-rolebinding.yaml
# ...and so on
```

## Verifying Your Solutions

After applying your solutions, verify them using the verification script:

```bash
cd ../../setup
./verify_solutions.sh
# Then select option 5 for Cluster Architecture
```

## Kubeadm and Cluster Management

For tasks related to kubeadm and cluster management that can't be directly performed in Minikube:

1. Document the commands you would use in a real environment
2. Create a text file with detailed steps and explanations
3. Reference the `kubeadm_practice_guide.md` file in the main directory for examples

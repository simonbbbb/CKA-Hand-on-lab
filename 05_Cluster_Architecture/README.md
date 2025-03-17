# Cluster Architecture, Installation and Configuration (25%)

This section covers setting up, configuring, and maintaining Kubernetes clusters.

## Key Concepts

- Role-Based Access Control (RBAC)
- Kubernetes cluster installation and configuration
- kubeadm for cluster management
- Cluster lifecycle management
- High availability control plane setup
- Helm and Kustomize for package management
- Container Network Interface (CNI)
- Container Storage Interface (CSI)
- Container Runtime Interface (CRI)
- Custom Resource Definitions (CRDs) and Operators

## Practice Questions

1. What are the core components of the Kubernetes control plane?
2. Explain the role of etcd in a Kubernetes cluster.
3. How would you set up RBAC to allow a user to only view pods in a specific namespace?
4. What steps are involved in creating a high-availability Kubernetes cluster with kubeadm?
5. How does the Container Network Interface (CNI) work in Kubernetes?
6. What is the purpose of Helm in a Kubernetes environment?
7. Explain the difference between Kustomize and Helm.
8. What are Custom Resource Definitions (CRDs) and how do they extend Kubernetes?
9. How would you upgrade a Kubernetes cluster with minimal downtime?
10. Explain the concept of operators in Kubernetes.

## Hands-on Tasks

### Task 1: Create RBAC Role

1. Create a custom role named `custom-role` in the `rbac-test` namespace
2. Configure the role to allow `get` permissions on pods
3. Verify your role configuration with: `kubectl get role custom-role -n rbac-test -o yaml`

### Task 2: Configure RBAC RoleBinding

1. Create a RoleBinding named `custom-binding` in the `rbac-test` namespace
2. Bind the `custom-role` you created in Task 1 to a User named `jane`
3. Verify the binding with: `kubectl get rolebinding custom-binding -n rbac-test -o yaml`

### Task 3: Create a Custom Resource Definition (CRD)

1. Create a Custom Resource Definition (CRD) named `backups.cka.training`
2. Define its schema to include fields for storage type and backup frequency
3. Verify your CRD with: `kubectl get crd backups.cka.training`

### Task 4: Create a kubeadm Configuration File

1. Create a file named `kubeadm-config.yaml` in the user_solutions directory
2. Configure it for a ClusterConfiguration that would initialize a Kubernetes control plane
3. Include settings for API server, networking, and etcd configuration

### Task 5: Document ETCD Backup Procedure

1. Create a file named `etcd-backup.md` in the user_solutions directory
2. Document the steps to backup etcd using etcdctl
3. Include the proper ETCDCTL_API version setting and command parameters
4. Specify necessary certificate paths and other required flags

### Task 6: Prepare Helm Chart Installation

1. Create a values.yaml file in the user_solutions directory for a Helm chart
2. Configure it with custom image settings and other deployment parameters
3. The file should include at least an image specification

### Task 7: Create Kustomize Configuration

1. Create a directory named `kustomize` in the user_solutions directory
2. Inside that directory, create a kustomization.yaml file
3. Configure it to customize a basic deployment by adding labels, namespace, or other settings
4. Ensure it contains the "kustomize" reference for validation

## Solutions

The solutions for these tasks can be found in the [solutions](./solutions/) directory.

## Additional Resources

- [Kubernetes Components](https://kubernetes.io/docs/concepts/overview/components/)
- [Installing Kubernetes with kubeadm](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/install-kubeadm/)
- [Highly Available Control Plane with kubeadm](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/high-availability/)
- [RBAC Authorization](https://kubernetes.io/docs/reference/access-authn-authz/rbac/)
- [Extending Kubernetes](https://kubernetes.io/docs/concepts/extend-kubernetes/)
- [Helm Documentation](https://helm.sh/docs/)
- [Kustomize Documentation](https://kustomize.io/)

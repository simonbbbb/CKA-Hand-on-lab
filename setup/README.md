# CKA Lab Environment Setup Guide

This guide helps you set up a local Kubernetes environment using Minikube for practicing CKA exam tasks. Minikube is a lightweight Kubernetes implementation that creates a VM on your local machine and deploys a simple, single-node cluster.

## Prerequisites

- MacOS, Linux, or Windows with WSL2
- At least 4GB of RAM and 20GB of disk space available
- Virtual machine capability (VirtualBox, Hyperkit, Docker, or similar)
- Terminal access

## Installation Steps

### 1. Install kubectl

`kubectl` is the Kubernetes command-line tool that allows you to run commands against Kubernetes clusters.

```bash
# For macOS using Homebrew
brew install kubectl

# Verify installation
kubectl version --client
```

### 2. Install Minikube

Minikube is a tool that makes it easy to run Kubernetes locally.

```bash
# For macOS using Homebrew
brew install minikube

# Verify installation
minikube version
```

### 3. Start Minikube

```bash
# Start with 4GB RAM, 2 CPUs, and the latest stable Kubernetes version
minikube start --memory=4096 --cpus=2

# Verify the cluster status
kubectl cluster-info
kubectl get nodes
```

## Lab Setup Scripts

Each lab section has its own setup script in this directory:

- `01_setup_storage_lab.sh`: Sets up the environment for storage exercises
- `02_setup_workloads_lab.sh`: Sets up the environment for workloads and scheduling exercises
- `03_setup_networking_lab.sh`: Sets up the environment for services and networking exercises
- `04_setup_troubleshooting_lab.sh`: Creates scenarios for troubleshooting practice
- `05_setup_cluster_arch_lab.sh`: Prepares an environment for cluster architecture tasks

### Text-based User Interface (TUI) Launcher

For easier management of lab environments, we provide a TUI launcher:

- `lab_launcher.sh`: Interactive menu to manage all aspects of the CKA lab environments

The TUI launcher provides the following features:
- Display current status of Minikube and available namespaces
- Launch individual lab setup scripts
- Reset the lab environment
- Check resources (pods, services, deployments) in each namespace
- View node status

To use the TUI launcher:
```bash
chmod +x ./lab_launcher.sh  # Make executable if needed
./lab_launcher.sh
```

### Manual Setup

Run the specific setup script before working on that section's tasks:

```bash
# Example: Setting up the storage lab environment
chmod +x ./01_setup_storage_lab.sh
./01_setup_storage_lab.sh
```

## Cleaning Up

When you're done with a lab session, you can either:

1. Stop Minikube (keeps the VM but stops it):
   ```bash
   minikube stop
   ```

2. Delete the Minikube cluster (removes everything):
   ```bash
   minikube delete
   ```

## Troubleshooting

- If Minikube fails to start, try specifying a driver:
  ```bash
  minikube start --driver=hyperkit  # For macOS
  minikube start --driver=docker    # If using Docker
  ```

- If a lab environment gets corrupted, you can reset it:
  ```bash
  ./reset_lab_environment.sh
  ```

## Additional Resources

- [Minikube Documentation](https://minikube.sigs.k8s.io/docs/)
- [kubectl Cheat Sheet](https://kubernetes.io/docs/reference/kubectl/cheatsheet/)

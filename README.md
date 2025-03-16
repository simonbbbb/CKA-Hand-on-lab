# CKA (Certified Kubernetes Administrator) Exam Preparation Lab

This repository contains practice questions, hands-on tasks, and resources to help you prepare for the CKA exam. The materials are organized according to the official CKA curriculum domains and their respective weights.


- [Medium Article](https://medium.com/@balazsdevops/preparing-for-the-new-cka-exam-a-hands-on-lab-environment-00b2b04c3c1f)
- [GitHub Repository](https://github.com/simonbbbb/CKA-Hand-on-lab)
- [CNCF CKA Curriculum](https://github.com/cncf/curriculum/blob/master/CKA_Curriculum_v1.32.pdf)

## Curriculum Domains

1. **Storage (10%)**
   - Implement storage classes and dynamic volume provisioning
   - Configure volume types, access modes, and reclaim policies
   - Manage persistent volumes and persistent volume claims

2. **Workloads and Scheduling (15%)**
   - Understand application deployments and how to perform rolling updates and rollbacks
   - Use ConfigMaps and Secrets to configure applications
   - Configure workload autoscaling
   - Understand the primitives used to create robust, self-healing, application deployments
   - Configure Pod admission and scheduling (limits, node affinity, etc.)

3. **Servicing and Networking (20%)**
   - Understand connectivity between Pods
   - Define and enforce Network Policies
   - Use ClusterIP, NodePort, LoadBalancer service types and endpoints
   - Use the Gateway API to manage Ingress traffic
   - Know how to use Ingress controllers and Ingress resources
   - Understand and use CoreDNS

4. **Troubleshooting (30%)**
   - Troubleshoot clusters and nodes
   - Troubleshoot cluster components
   - Monitor cluster and application resource usage
   - Manage and evaluate container output streams
   - Troubleshoot services and networking

5. **Cluster Architecture, Installation and Configuration (25%)**
   - Manage role-based access control (RBAC)
   - Prepare underlying infrastructure for installing a Kubernetes cluster
   - Create and manage Kubernetes clusters using kubeadm
   - Manage the lifecycle of Kubernetes clusters
   - Implement and configure a highly-available control plane
   - Use Helm and Kustomize to install cluster components
   - Understand extension interfaces (CNI, CSI, CRI, etc.)
   - Understand CRDs, install and configure operators

## Setup Instructions

### Prerequisites

- Basic knowledge of Linux and container concepts
- Terminal/Command Line familiarity
- Git installed

### Environment Setup Options

#### Option 1: Using Docker (Most Common)

1. **Requirements:**
   - Docker Desktop installed and running
   - `kubectl` command-line tool
   - Minikube

2. **Setup Steps:**
   ```bash
   # Start Minikube with Docker driver
   cd setup
   ./reset_lab_environment.sh  # This will initialize Minikube with Docker driver
   ```

#### Option 2: Using macOS with Apple Silicon (M1/M2/M3)

1. **Requirements:**
   - QEMU installed (`brew install qemu`)
   - `kubectl` command-line tool
   - Minikube

2. **Setup Steps:**
   ```bash
   # Start Minikube with QEMU driver
   cd setup
   ./reset_lab_environment.sh  # The script is configured to use qemu2 driver for Apple Silicon
   ```

#### Option 3: Using any other environment

1. **Requirements:**
   - Any Kubernetes cluster (cloud-based GKE, AKS, EKS or local Kind, k3s, etc.)
   - `kubectl` configured to access your cluster

2. **Setup Steps:**
   - Skip the Minikube parts of the setup scripts
   - Manually apply the Kubernetes resources from each section

### Setting Up Lab Environments

The repository contains setup scripts for each domain:

```bash
cd setup
./01_setup_storage_lab.sh        # Sets up Storage lab environment
./02_setup_workloads_lab.sh      # Sets up Workloads lab environment
./03_setup_networking_lab.sh     # Sets up Networking lab environment
./04_setup_troubleshooting_lab.sh # Sets up Troubleshooting lab environment
./05_setup_cluster_arch_lab.sh   # Sets up Cluster Architecture lab environment
```

To reset your environment at any time:

```bash
cd setup
./reset_lab_environment.sh
```

## How to Use This Repository

### Directory Structure

```
CKA_LAB/
├── 01_Storage/                # Storage domain exercises
│   ├── README.md              # Task instructions
│   ├── solutions/             # Official solution files
│   └── user_solutions/        # Your solutions (gitignored)
├── 02_Workloads/              # Workloads domain exercises
│   ├── ...
├── 03_Networking/             # Networking domain exercises
│   ├── ...
├── 04_Troubleshooting/        # Troubleshooting domain exercises
│   ├── ...
├── 05_Cluster_Architecture/   # Cluster Architecture domain exercises
│   ├── ...
└── setup/                     # Setup scripts
    ├── 01_setup_storage_lab.sh
    ├── ...
    ├── reset_lab_environment.sh
    └── verify_solutions.sh    # Script to verify your solutions
```

### Recommended Workflow

1. **Prepare your environment:**
   ```bash
   cd setup
   ./reset_lab_environment.sh
   ```

2. **Start with a section:**
   - Read the README.md in the section directory
   - Run the corresponding setup script: `./setup/01_setup_storage_lab.sh`

3. **Complete the hands-on tasks:**
   - Create your solution files in the `user_solutions` directory
   - Apply your solutions to the cluster with `kubectl apply`

4. **Verify your solutions:**
   ```bash
   cd setup
   ./verify_solutions.sh
   # Select the appropriate section to verify
   ```

5. **Compare with official solutions:**
   - Only after attempting the tasks, review the official solutions

6. **Reset and move to the next section:**
   ```bash
   cd setup
   ./reset_lab_environment.sh
   ./02_setup_workloads_lab.sh  # For the next section
   ```

### Important Notes

1. **User Solutions:** 
   - Each section has a `user_solutions/` directory where you should save your YAML files
   - These directories are gitignored, so you won't accidentally commit your solutions

2. **Setup Scripts:**
   - The setup scripts create the necessary infrastructure but don't provide complete solutions
   - They may create some resources that are similar to what you need to create in the tasks

3. **Verification:**
   - Use the `verify_solutions.sh` script to check if your solutions meet the requirements
   - The script will provide feedback on what's correct and what needs fixing

### Helm and Kustomize Tasks

The updated CKA exam (February 2025) includes tasks related to Helm and Kustomize for package management. In the Cluster Architecture section, you'll find:

1. **Helm Tasks:**
   - Creating and customizing Helm chart values
   - Installing applications using Helm charts
   - Managing releases and upgrades

2. **Kustomize Tasks:**
   - Using Kustomize to manage Kubernetes manifests
   - Creating overlays for different environments
   - Patching resources with Kustomize

## Troubleshooting Common Issues

### Minikube Issues on macOS Silicon

If you encounter issues with Minikube on Apple Silicon:

1. **Docker Driver Not Working:**
   ```bash
   # Make sure to install QEMU
   brew install qemu
   
   # Use the qemu2 driver instead
   minikube delete
   minikube start --driver=qemu2 --memory=4096 --cpus=2
   ```

2. **Slow Performance:**
   - Increase the memory and CPU allocation in the scripts
   - Modify the line: `minikube start --memory=6144 --cpus=4 --driver=qemu2`

### Kubernetes Resource Issues

If you encounter issues with Kubernetes resources:

1. **Resource not found:**
   ```bash
   # Check if the namespace exists
   kubectl get namespaces
   
   # Ensure you're in the correct namespace
   kubectl config set-context --current --namespace=<namespace>
   ```

2. **Permission issues:**
   ```bash
   # Check RBAC settings
   kubectl auth can-i create deployments
   kubectl auth can-i create pods
   ```

## Additional Resources

- [Kubernetes Documentation](https://kubernetes.io/docs/home/)
- [CKA Curriculum](https://github.com/cncf/curriculum)
- [CNCF Certification Information](https://www.cncf.io/certification/cka/)

## About the Author

Visit my personal page projects and blog:

- Personal Site: [www.siminbalazs.hu](http://www.siminbalazs.hu)
- GitHub: [github.com/simonbbbb](https://github.com/simonbbbb)
- LinkedIn: [linkedin.com/in/simonbalazshu](https://www.linkedin.com/in/simonbalazshu)
- Medium: [medium.com/@balazsdevops](https://medium.com/@balazsdevops)

Feel free to reach out with questions about the lab environment or Kubernetes in general!

Good luck with your CKA exam preparation!

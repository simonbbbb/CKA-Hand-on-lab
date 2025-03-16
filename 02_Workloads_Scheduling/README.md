# Workloads and Scheduling (15%)

This section covers deploying applications, configuring them, and controlling how they are scheduled across the cluster.

## Key Concepts

- Deployments, StatefulSets, DaemonSets
- Rolling Updates and Rollbacks
- ConfigMaps and Secrets
- Workload Autoscaling (HPA)
- Self-healing applications
- Pod scheduling (resource limits, node affinity, taints and tolerations)

## Practice Questions

1. What is the difference between a rolling update and a recreate deployment strategy?
2. How can you view the rollout history of a deployment?
3. How do you roll back a deployment to a previous revision?
4. What is the purpose of a Pod Disruption Budget?
5. Explain how horizontal pod autoscaling works in Kubernetes.
6. What is the difference between node affinity and taints/tolerations?
7. How can you ensure that a pod is scheduled on a specific node?
8. What happens when a pod exceeds its resource limits?

## Hands-on Tasks

### Task 1: Create a Deployment with Rolling Update Strategy

Create a deployment with the following specifications:
- Name: `nginx-deployment`
- 3 replicas of `nginx:1.14.2`
- Rolling update strategy with max unavailable 1 and max surge 1
- Label selector: `app=nginx`

### Task 2: Perform a Rolling Update

Update the deployment created in Task 1 to use `nginx:1.16.1` and observe the rolling update process.

### Task 3: Roll Back a Deployment

Roll back the deployment to the previous version.

### Task 4: Configure Applications with ConfigMaps and Secrets

1. Create a ConfigMap named `app-config` with the following data:
   - `DATABASE_URL=mysql://db:3306/mydb`
   - `DEBUG_MODE=true`

2. Create a Secret named `app-secret` with the following data:
   - `username=admin`
   - `password=supersecret`

3. Create a Pod that uses both the ConfigMap and Secret.

### Task 5: Implement Horizontal Pod Autoscaling

Set up a Horizontal Pod Autoscaler for a deployment that scales based on CPU usage.

### Task 6: Configure Pod Scheduling

1. Label a node with `disk=ssd`
2. Create a pod that uses node affinity to ensure it's scheduled on the node with the `disk=ssd` label.
3. Taint a node with `app=critical:NoSchedule`
4. Create a pod that tolerates this taint.

### Task 7: Resource Limits and Requests

Create a Pod with the following resource requirements:
- Requests: 256Mi memory, 0.5 CPU
- Limits: 512Mi memory, 1 CPU

## Solutions

The solutions for these tasks can be found in the [solutions](./solutions/) directory.

## Additional Resources

- [Kubernetes Deployments](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/)
- [ConfigMaps and Secrets](https://kubernetes.io/docs/concepts/configuration/)
- [Horizontal Pod Autoscaling](https://kubernetes.io/docs/tasks/run-application/horizontal-pod-autoscale/)
- [Assigning Pods to Nodes](https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/)
- [Taints and Tolerations](https://kubernetes.io/docs/concepts/scheduling-eviction/taint-and-toleration/)

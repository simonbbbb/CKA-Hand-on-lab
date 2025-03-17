# Troubleshooting (30%)

This section covers diagnosing and resolving issues with Kubernetes clusters, applications, and networking.

## Key Concepts

- Cluster-level troubleshooting
- Node troubleshooting
- Control plane component troubleshooting
- Application troubleshooting
- Networking troubleshooting
- Resource monitoring and logging
- Container output analysis

## Practice Questions

1. What command would you use to check the status of all nodes in a cluster?
2. How can you check the logs of a control plane component running as a static pod?
3. What steps would you take to troubleshoot a pod that is stuck in "Pending" state?
4. How would you troubleshoot a Service that isn't routing traffic correctly to its pods?
5. What would you check if a pod is in the "CrashLoopBackOff" state?
6. How can you monitor resource usage at the node level?
7. How would you check the events related to a specific resource in Kubernetes?
8. What command helps you identify why a pod cannot be scheduled?

## Hands-on Tasks

### Task 1: Troubleshoot a Pod with Image Pull Error

1. Examine the `broken-pod` that is failing to start
2. Identify why the image cannot be pulled
3. Fix the issue and verify the pod starts correctly

### Task 2: Identify and Resolve Resource Constraints

1. Examine the `resource-constrained-pod` 
2. Determine if it's experiencing resource issues
3. Adjust the resource allocation if needed

### Task 3: Fix Pod Configuration Issues

1. Examine the `config-error-pod` with configuration problems
2. Identify the missing ConfigMap issue
3. Create the required ConfigMap and fix the pod

### Task 4: Debug Service Connectivity Issues

1. Investigate why the `web-service` isn't routing traffic to the `web-deployment` pods
2. Identify the service selector mismatch
3. Correct the service configuration and verify connectivity

### Task 5: Resolve Deployment Update Problems

1. Examine the `update-issue-deployment` that can't successfully roll out
2. Identify the readiness probe issue causing the rollout to fail
3. Fix the deployment configuration to allow successful updates

### Task 6: Understand Node-level Troubleshooting

1. Review the node troubleshooting guide in `solutions/node_troubleshooting.md`
2. Understand common node failure scenarios and resolution approaches
3. Practice the commands you would use in a real-world scenario

### Task 7: Troubleshoot DNS Resolution

1. Use the `dns-client` pod to try to connect to the `dns-service`
2. Identify why the DNS resolution or service connection is failing
3. Fix the service selector issue and verify DNS resolution works

## Solutions

The solutions for these tasks can be found in the [solutions](./solutions/) directory.

## Additional Resources

- [Kubernetes Troubleshooting Guide](https://kubernetes.io/docs/tasks/debug/)
- [Debug Pods and ReplicaSets](https://kubernetes.io/docs/tasks/debug/debug-application/debug-pods-replicasets/)
- [Debug Services](https://kubernetes.io/docs/tasks/debug/debug-application/debug-service/)
- [Debugging DNS Resolution](https://kubernetes.io/docs/tasks/administer-cluster/dns-debugging-resolution/)
- [Resource Metrics Pipeline](https://kubernetes.io/docs/tasks/debug/debug-cluster/resource-metrics-pipeline/)

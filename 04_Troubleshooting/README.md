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

### Task 1: Fix Pod with Image Pull Error

1. Examine the `broken-pod` in the troubleshooting namespace
2. Identify why the image cannot be pulled (nonexistent version tag)
3. Fix the issue by updating to a valid image version (e.g., nginx:latest)
4. Verify the pod is now in the Running state with: `kubectl get pod broken-pod -n troubleshooting -o jsonpath='{.status.phase}'`

### Task 2: Fix Resource-Constrained Pod

1. Examine the `resource-constrained-pod` in the troubleshooting namespace 
2. Determine why it's having issues (limited memory/CPU resources)
3. Modify its resource allocation to appropriate values
4. Verify the pod is now in the Running state with: `kubectl get pod resource-constrained-pod -n troubleshooting -o jsonpath='{.status.phase}'`

### Task 3: Fix Service Connectivity Issues

1. Investigate why the `web-service` isn't routing traffic to the `web-deployment` pods
2. Identify the service selector mismatch issue
3. Correct the frontend-service selector to properly match the pod labels
4. Verify endpoints exist with: `kubectl get endpoints frontend-service -n troubleshooting`

### Task 4: Fix ConfigMap Error

1. Examine the `config-error-pod` with configuration problems
2. Identify the missing ConfigMap issue (nonexistent-config)
3. Create the required ConfigMap named `app-config` with a DATABASE_URL key
4. Set the DATABASE_URL value to: "mysql://user:password@db:3306/app"
5. Verify with: `kubectl get configmap app-config -n troubleshooting -o jsonpath='{.data.DATABASE_URL}'`

### Task 5: Fix Deployment Update Strategy

1. Examine the deployment named `web-app` in the troubleshooting namespace
2. Identify issues with its update strategy
3. Modify the deployment to use a RollingUpdate strategy
4. Verify with: `kubectl get deployment web-app -n troubleshooting -o jsonpath='{.spec.strategy.type}'`

## Solutions

The solutions for these tasks can be found in the [solutions](./solutions/) directory.

## Additional Resources

- [Kubernetes Troubleshooting Guide](https://kubernetes.io/docs/tasks/debug/)
- [Debug Pods and ReplicaSets](https://kubernetes.io/docs/tasks/debug/debug-application/debug-pods-replicasets/)
- [Debug Services](https://kubernetes.io/docs/tasks/debug/debug-application/debug-service/)
- [Debugging DNS Resolution](https://kubernetes.io/docs/tasks/administer-cluster/dns-debugging-resolution/)
- [Resource Metrics Pipeline](https://kubernetes.io/docs/tasks/debug/debug-cluster/resource-metrics-pipeline/)

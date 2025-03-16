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

### Task 1: Troubleshoot a Failed Node

1. Identify a node that is in NotReady state
2. Diagnose the issue (could be kubelet not running, networking issues, etc.)
3. Resolve the issue to bring the node back to Ready state

### Task 2: Debug a Failing Pod

1. Deploy a pod that fails to start due to an invalid configuration
2. Use kubectl commands to identify the problem
3. Fix the issue and verify the pod starts correctly

### Task 3: Troubleshoot Control Plane Components

1. Check the status of all control plane components
2. Identify a non-functioning component
3. Check logs to determine the cause
4. Restore the component to a working state

### Task 4: Identify Resource Constraints

1. Deploy a resource-intensive application
2. Use metrics tools to identify resource bottlenecks
3. Implement appropriate resource limits

### Task 5: Debug Service Connectivity Issues

1. Deploy an application with a service that isn't working
2. Debug and fix service endpoint issues
3. Verify traffic flows correctly

### Task 6: Analyze Container Logs

1. Deploy an application that produces error logs
2. Extract and analyze the relevant logs
3. Identify and resolve the underlying issue

### Task 7: Troubleshoot DNS Resolution

1. Deploy two pods: a client and a server
2. The client can't connect to the server using DNS name
3. Diagnose and fix the DNS resolution issue

## Solutions

The solutions for these tasks can be found in the [solutions](./solutions/) directory.

## Additional Resources

- [Kubernetes Troubleshooting Guide](https://kubernetes.io/docs/tasks/debug/)
- [Debug Pods and ReplicaSets](https://kubernetes.io/docs/tasks/debug/debug-application/debug-pods-replicasets/)
- [Debug Services](https://kubernetes.io/docs/tasks/debug/debug-application/debug-service/)
- [Debugging DNS Resolution](https://kubernetes.io/docs/tasks/administer-cluster/dns-debugging-resolution/)
- [Resource Metrics Pipeline](https://kubernetes.io/docs/tasks/debug/debug-cluster/resource-metrics-pipeline/)

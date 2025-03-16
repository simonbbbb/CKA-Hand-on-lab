# Servicing and Networking (20%)

This section covers Kubernetes networking concepts, service discovery, and traffic management.

## Key Concepts

- Pod-to-Pod communication
- Network Policies
- Service types (ClusterIP, NodePort, LoadBalancer)
- Service Endpoints
- Gateway API and Ingress resources
- Ingress Controllers
- CoreDNS for service discovery

## Practice Questions

1. Explain how Pod-to-Pod communication works in Kubernetes.
2. What is the purpose of a Network Policy?
3. Describe the difference between ClusterIP, NodePort, and LoadBalancer service types.
4. How do you restrict traffic to a Pod using a Network Policy?
5. What is the role of CoreDNS in a Kubernetes cluster?
6. How do services map to endpoints in Kubernetes?
7. Explain the difference between an Ingress Controller and an Ingress Resource.
8. How does the Gateway API differ from traditional Ingress resources?

## Hands-on Tasks

### Task 1: Create Different Service Types

1. Create a Deployment with 3 replicas of nginx.
2. Create a ClusterIP Service that exposes this Deployment.
3. Create a NodePort Service that exposes this Deployment.
4. If your environment supports it, create a LoadBalancer Service.

### Task 2: Implement Network Policies

Create a Network Policy that:
1. Allows traffic to a Pod with label `app=db` only from Pods with label `app=web`
2. Allows egress traffic only to port 53 (DNS) and to the Kubernetes API server

### Task 3: Set Up an Ingress Resource

1. Deploy an Ingress Controller (if not already available in your environment)
2. Create an Ingress Resource that routes:
   - Traffic for `myapp.example.com/api` to a service called `api-service`
   - Traffic for `myapp.example.com` to a service called `web-service`

### Task 4: Troubleshoot Service Connectivity

1. Deploy two Pods: a client and a server
2. Create a Service for the server
3. Debug connectivity issues between the client and server

### Task 5: Implement Multi-Port Services

Create a Service that exposes multiple ports from a single Pod:
- Port 80 for HTTP
- Port 443 for HTTPS

### Task 6: CoreDNS Configuration

Examine the CoreDNS configuration in your cluster and modify it to implement a custom domain resolution.

## Solutions

The solutions for these tasks can be found in the [solutions](./solutions/) directory.

## Additional Resources

- [Kubernetes Networking](https://kubernetes.io/docs/concepts/services-networking/)
- [Network Policies](https://kubernetes.io/docs/concepts/services-networking/network-policies/)
- [Service Types](https://kubernetes.io/docs/concepts/services-networking/service/)
- [Ingress](https://kubernetes.io/docs/concepts/services-networking/ingress/)
- [DNS for Services and Pods](https://kubernetes.io/docs/concepts/services-networking/dns-pod-service/)

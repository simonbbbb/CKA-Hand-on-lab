# Troubleshooting User Solutions

Place your troubleshooting solution YAML files and notes in this directory. This directory is gitignored, so your solutions won't be committed to the repository.

## Recommended File Naming

For each scenario, create a separate YAML file with a descriptive name and a notes file:

- Scenario 1: 
  - `01-broken-pod-fix.yaml` 
  - `01-broken-pod-notes.md` (Document what was wrong and how you fixed it)

- Scenario 2: 
  - `02-resource-constrained-fix.yaml`
  - `02-resource-constrained-notes.md`

- Scenario 3: 
  - `03-config-error-fix.yaml`
  - `03-config-error-notes.md`

- Scenario 4: 
  - `04-service-selector-fix.yaml`
  - `04-service-selector-notes.md`

- Scenario 5: 
  - `05-deployment-update-fix.yaml`
  - `05-deployment-update-notes.md`

## Applying Your Solutions

Apply your solutions to the Kubernetes cluster:

```bash
kubectl apply -f 01-broken-pod-fix.yaml
kubectl apply -f 02-resource-constrained-fix.yaml
# ...and so on
```

## Verifying Your Solutions

After applying your solutions, verify them using the verification script:

```bash
cd ../../setup
./verify_solutions.sh
# Then select option 4 for Troubleshooting
```

## Important Troubleshooting Tips

1. Use `kubectl describe` to get detailed information about resources
2. Check logs with `kubectl logs`
3. Look at events with `kubectl get events`
4. For pod issues, check its status, readiness, and liveness probes
5. For service issues, ensure selectors match pod labels

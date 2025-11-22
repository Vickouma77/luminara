# Luminara GitOps Setup Guide

## Prerequisites

- Kubernetes cluster (1.24+)
- kubectl configured
- ArgoCD installed
- GitHub repository access
- GitHub Container Registry access

## Quick Start

### 1. Install ArgoCD

```bash
# Create argocd namespace
kubectl create namespace argocd

# Install ArgoCD
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml

# Wait for ArgoCD to be ready
kubectl wait --for=condition=available --timeout=300s deployment/argocd-server -n argocd

# Get initial admin password
kubectl -n argocd get secret argocd-initial-admin-secret -o jsonpath="{.data.password}" | base64 -d

# Port forward to access UI
kubectl port-forward svc/argocd-server -n argocd 8080:443
```

Access ArgoCD UI at https://localhost:8080

### 2. Configure GitHub Repository

```bash
# Add repository credentials
argocd repo add https://github.com/Vickouma77/luminara.git \
  --username <github-username> \
  --password <github-token>
```

### 3. Create Luminara Project

```bash
kubectl apply -f argocd/project.yaml
```

### 4. Deploy Applications

#### Development Environment

```bash
kubectl apply -f argocd/application-dev.yaml

# Sync application
argocd app sync luminara-dev
```

#### Staging Environment

```bash
kubectl apply -f argocd/application-staging.yaml

# Manual sync required
argocd app sync luminara-staging
```

#### Production Environment

```bash
kubectl apply -f argocd/application-production.yaml

# Production requires manual sync with confirmation
argocd app sync luminara-prod --prune
```

## GitHub Actions Setup

### 1. Create GitHub Secrets

Navigate to your repository → Settings → Secrets and add:

```
GITHUB_TOKEN (automatically available)
SLACK_WEBHOOK (optional, for notifications)
SLACK_WEBHOOK_CRITICAL (for production alerts)
```

### 2. Enable GitHub Container Registry

```bash
# Login to GHCR
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin

# Tag and push images
docker tag luminara-api-gateway ghcr.io/vickouma77/luminara-api-gateway:latest
docker push ghcr.io/vickouma77/luminara-api-gateway:latest
```

### 3. Configure Kustomize

Install kustomize:

```bash
curl -s "https://raw.githubusercontent.com/kubernetes-sigs/kustomize/master/hack/install_kustomize.sh"  | bash
sudo mv kustomize /usr/local/bin/
```

## Deployment Workflows

### Development Deployment (Automatic)

1. Push code to `main` branch
2. GitHub Actions runs CI pipeline
3. Build and push images to GHCR
4. Update dev manifests automatically
5. ArgoCD auto-syncs to dev namespace

### Staging Deployment (Manual Approval)

1. Create release branch: `git checkout -b release/1.0.0`
2. Push to GitHub
3. GitHub Actions creates staging PR
4. Review and merge PR
5. Sync in ArgoCD UI or CLI

```bash
argocd app sync luminara-staging
```

### Production Deployment (Controlled)

1. Trigger production workflow:

```bash
# Via GitHub UI: Actions → CD - Production Deployment → Run workflow
# Or via CLI:
gh workflow run cd-production.yml -f version=1.0.0 -f deployment_strategy=rolling
```

2. Review production PR (requires 2+ approvals)
3. Merge PR after approval
4. Manual sync in ArgoCD:

```bash
argocd app sync luminara-prod --prune --force
```

5. Monitor deployment:

```bash
argocd app wait luminara-prod --health
kubectl rollout status deployment/api-gateway -n luminara-prod
```

## Monitoring Deployments

### ArgoCD CLI

```bash
# View application status
argocd app get luminara-prod

# View sync history
argocd app history luminara-prod

# View application logs
argocd app logs luminara-prod

# Rollback to previous version
argocd app rollback luminara-prod
```

### Kubernetes CLI

```bash
# Watch deployment progress
kubectl get pods -n luminara-prod -w

# Check deployment status
kubectl rollout status deployment/api-gateway -n luminara-prod

# View logs
kubectl logs -f deployment/api-gateway -n luminara-prod

# Rollback deployment
kubectl rollout undo deployment/api-gateway -n luminara-prod
```

## Troubleshooting

### Application Out of Sync

```bash
# Hard refresh
argocd app get luminara-dev --hard-refresh

# Sync with force
argocd app sync luminara-dev --force

# Prune extra resources
argocd app sync luminara-dev --prune
```

### Image Pull Errors

```bash
# Create image pull secret
kubectl create secret docker-registry ghcr-secret \
  --docker-server=ghcr.io \
  --docker-username=<github-username> \
  --docker-password=<github-token> \
  -n luminara-prod

# Add to deployment
kubectl patch serviceaccount default -n luminara-prod \
  -p '{"imagePullSecrets": [{"name": "ghcr-secret"}]}'
```

### Sync Failures

```bash
# View sync errors
argocd app get luminara-prod

# View detailed diff
argocd app diff luminara-prod

# Skip validation
argocd app sync luminara-prod --validate=false
```

## Best Practices

### 1. Version Tagging

- Development: `dev-<sha>`
- Staging: `staging-<version>`
- Production: `v<version>`

### 2. Rollout Strategy

- Development: Fast rollout, auto-sync
- Staging: Controlled rollout, manual approval
- Production: Gradual rollout, manual sync only

### 3. Monitoring

- Set up Grafana dashboards
- Configure Prometheus alerts
- Enable ArgoCD notifications

### 4. Secrets Management

- Use Sealed Secrets or External Secrets Operator
- Never commit secrets to Git
- Rotate credentials regularly

### 5. Disaster Recovery

- Regular backups of ArgoCD config
- Document rollback procedures
- Test disaster recovery quarterly

## Advanced Configuration

### Blue-Green Deployments

```bash
# Deploy to blue environment
kubectl apply -f k8s/overlays/production-blue/

# Test blue environment
curl https://blue.luminara.io/api/v1/health

# Switch traffic
kubectl patch service api-gateway -n luminara-prod \
  -p '{"spec":{"selector":{"version":"blue"}}}'
```

### Canary Deployments with Flagger

```yaml
apiVersion: flagger.app/v1beta1
kind: Canary
metadata:
  name: api-gateway
spec:
  targetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: api-gateway
  service:
    port: 8000
  analysis:
    interval: 1m
    threshold: 5
    maxWeight: 50
    stepWeight: 10
```

## Security Checklist

- [ ] Enable RBAC in ArgoCD
- [ ] Configure SSO (GitHub/Google)
- [ ] Enable webhook verification
- [ ] Set up image signing with Cosign
- [ ] Configure network policies
- [ ] Enable pod security policies
- [ ] Regular security scans
- [ ] Audit log monitoring

## Support

For issues or questions:
- GitHub Issues: https://github.com/Vickouma77/luminara/issues
- Documentation: /docs/
- ArgoCD Docs: https://argo-cd.readthedocs.io/

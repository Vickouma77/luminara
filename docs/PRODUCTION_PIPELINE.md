# Luminara Production Pipeline - GitOps Architecture

## Overview

This document outlines the production deployment strategy for Luminara banking platform using GitOps principles with ArgoCD.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         GitHub Repository                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Source     │  │  Kubernetes  │  │   Helm       │         │
│  │   Code       │  │  Manifests   │  │   Charts     │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
         │                    │                    │
         │ Push               │ Monitor            │
         ▼                    ▼                    │
┌─────────────────┐  ┌─────────────────┐         │
│  GitHub Actions │  │     ArgoCD      │◄────────┘
│                 │  │                 │
│  • Build        │  │  • Sync         │
│  • Test         │  │  • Deploy       │
│  • Push Images  │  │  • Monitor      │
│  • Update Manifests│ │  • Rollback  │
└─────────────────┘  └─────────────────┘
         │                    │
         │                    │ Apply
         ▼                    ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Container Registry (GHCR)                     │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ Pull Images
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Kubernetes Cluster                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │ Development │  │   Staging   │  │  Production │            │
│  │  Namespace  │  │  Namespace  │  │  Namespace  │            │
│  └─────────────┘  └─────────────┘  └─────────────┘            │
└─────────────────────────────────────────────────────────────────┘
```

## Environments

### 1. Development
- Namespace: `luminara-dev`
- Auto-sync: Enabled
- Purpose: Continuous integration testing
- Database: Shared PostgreSQL with dev database

### 2. Staging
- Namespace: `luminara-staging`
- Auto-sync: Manual approval required
- Purpose: Pre-production testing
- Database: Dedicated PostgreSQL instance

### 3. Production
- Namespace: `luminara-prod`
- Auto-sync: Disabled (manual only)
- Purpose: Live customer traffic
- Database: High-availability PostgreSQL cluster
- Monitoring: Full observability stack

## GitOps Workflow

### Development Flow
1. Developer pushes code to feature branch
2. GitHub Actions runs tests
3. On merge to `main`:
   - Build Docker images
   - Push to GHCR with `dev-<sha>` tag
   - Update dev manifests
   - ArgoCD auto-syncs to dev namespace

### Staging Flow
1. Create release branch or tag
2. GitHub Actions builds images with `staging-<version>` tag
3. Update staging manifests
4. Approve ArgoCD sync via PR review
5. ArgoCD deploys to staging

### Production Flow
1. Create production release tag (e.g., `v1.0.0`)
2. GitHub Actions builds images with version tag
3. Create PR to update production manifests
4. Require approvals (2+ reviewers)
5. Manual ArgoCD sync to production
6. Monitor rollout with gradual traffic shift

## Key Components

### CI Pipeline (GitHub Actions)
- **Triggers**: Push, PR, Tag
- **Jobs**:
  - Lint & Format Check
  - Unit Tests
  - Integration Tests
  - Security Scanning (Trivy)
  - Build Docker Images
  - Push to Registry
  - Update Kubernetes Manifests

### CD Pipeline (ArgoCD)
- **Sync Strategy**: Git as source of truth
- **Health Checks**: Pod readiness/liveness
- **Rollback**: Automatic on failure
- **Notifications**: Slack/Email alerts

### Container Registry
- **Provider**: GitHub Container Registry (GHCR)
- **Tagging Strategy**:
  - `latest`: Latest main build
  - `dev-<sha>`: Development builds
  - `staging-<version>`: Staging releases
  - `v<semver>`: Production releases
  - `<sha>`: Git commit SHA for traceability

## Deployment Strategy

### Rolling Updates
- Max surge: 25%
- Max unavailable: 0%
- Ensures zero-downtime deployments

### Blue-Green Deployments (Production)
- Maintain two identical environments
- Switch traffic via Ingress
- Quick rollback capability

### Canary Deployments (Optional)
- Progressive delivery with Flagger
- Automated rollback on metrics degradation
- 10% → 25% → 50% → 100% traffic shift

## Security

### Image Scanning
- Trivy scans on every build
- Block deployment on critical vulnerabilities
- SBOM generation

### Secrets Management
- Sealed Secrets for GitOps
- External Secrets Operator for cloud secrets
- Rotate credentials regularly

### RBAC
- Least privilege access
- Separate service accounts per service
- Pod security policies enforced

## Monitoring & Observability

### Metrics
- Prometheus for metrics collection
- Grafana dashboards
- Custom SLIs/SLOs

### Logging
- Loki for log aggregation
- Structured JSON logs
- Log retention policies

### Tracing
- Jaeger for distributed tracing
- OpenTelemetry instrumentation
- Trace sampling in production

### Alerting
- PagerDuty/Opsgenie integration
- Multi-channel notifications
- Escalation policies

## Disaster Recovery

### Backup Strategy
- Database: Daily backups with point-in-time recovery
- Configs: GitOps repository is source of truth
- Secrets: Encrypted backups in secure storage

### Recovery Procedures
- RTO: 30 minutes
- RPO: 1 hour
- Automated failover for critical services
- Regular DR drills

## Cost Optimization

- Horizontal Pod Autoscaling (HPA)
- Vertical Pod Autoscaling (VPA)
- Cluster autoscaling
- Resource quotas per namespace
- Spot instances for non-critical workloads

## Compliance

- Audit logs for all deployments
- Immutable infrastructure
- Change approval workflows
- Compliance as code (OPA policies)

## Migration Plan

### Phase 1: Setup (Week 1)
- [ ] Install ArgoCD
- [ ] Configure GitHub Actions
- [ ] Setup GHCR
- [ ] Create base Kubernetes manifests

### Phase 2: Development Environment (Week 2)
- [ ] Deploy to dev namespace
- [ ] Configure auto-sync
- [ ] Validate CI/CD pipeline

### Phase 3: Staging Environment (Week 3)
- [ ] Setup staging namespace
- [ ] Configure manual approval
- [ ] End-to-end testing

### Phase 4: Production Readiness (Week 4)
- [ ] Production namespace setup
- [ ] Security hardening
- [ ] Monitoring & alerting
- [ ] Load testing

### Phase 5: Go Live (Week 5)
- [ ] Blue-green production deployment
- [ ] Traffic migration
- [ ] 24/7 monitoring
- [ ] Post-deployment validation

#!/bin/bash

# Luminara GitOps Deployment Script
# This script helps deploy Luminara to different environments using ArgoCD

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
ARGOCD_NAMESPACE="argocd"
PROJECT_NAME="luminara"

# Functions
print_header() {
    echo -e "${GREEN}================================${NC}"
    echo -e "${GREEN}$1${NC}"
    echo -e "${GREEN}================================${NC}"
}

print_info() {
    echo -e "${YELLOW}➜ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

check_prerequisites() {
    print_header "Checking Prerequisites"
    
    # Check kubectl
    if ! command -v kubectl &> /dev/null; then
        print_error "kubectl not found. Please install kubectl."
        exit 1
    fi
    print_success "kubectl found"
    
    # Check argocd CLI
    if ! command -v argocd &> /dev/null; then
        print_error "argocd CLI not found. Please install it:"
        echo "  brew install argocd"
        echo "  or download from: https://argo-cd.readthedocs.io/en/stable/cli_installation/"
        exit 1
    fi
    print_success "argocd CLI found"
    
    # Check kustomize
    if ! command -v kustomize &> /dev/null; then
        print_error "kustomize not found. Please install it:"
        echo "  brew install kustomize"
        exit 1
    fi
    print_success "kustomize found"
    
    # Check cluster access
    if ! kubectl cluster-info &> /dev/null; then
        print_error "Cannot access Kubernetes cluster"
        exit 1
    fi
    print_success "Kubernetes cluster accessible"
}

install_argocd() {
    print_header "Installing ArgoCD"
    
    if kubectl get namespace $ARGOCD_NAMESPACE &> /dev/null; then
        print_info "ArgoCD namespace already exists"
    else
        print_info "Creating ArgoCD namespace..."
        kubectl create namespace $ARGOCD_NAMESPACE
    fi
    
    print_info "Applying ArgoCD manifests..."
    kubectl apply -n $ARGOCD_NAMESPACE -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml
    
    print_info "Waiting for ArgoCD to be ready..."
    kubectl wait --for=condition=available --timeout=300s deployment/argocd-server -n $ARGOCD_NAMESPACE
    
    print_success "ArgoCD installed successfully"
    
    # Get initial admin password
    print_info "Retrieving ArgoCD admin password..."
    ARGOCD_PASSWORD=$(kubectl -n $ARGOCD_NAMESPACE get secret argocd-initial-admin-secret -o jsonpath="{.data.password}" | base64 -d)
    
    echo ""
    echo -e "${GREEN}ArgoCD Admin Credentials:${NC}"
    echo "  Username: admin"
    echo "  Password: $ARGOCD_PASSWORD"
    echo ""
    echo "Access ArgoCD UI:"
    echo "  kubectl port-forward svc/argocd-server -n $ARGOCD_NAMESPACE 8080:443"
    echo "  Then visit: https://localhost:8080"
}

login_argocd() {
    print_header "Logging into ArgoCD"
    
    # Port forward in background
    kubectl port-forward svc/argocd-server -n $ARGOCD_NAMESPACE 8080:443 &> /dev/null &
    PF_PID=$!
    sleep 3
    
    # Get password
    ARGOCD_PASSWORD=$(kubectl -n $ARGOCD_NAMESPACE get secret argocd-initial-admin-secret -o jsonpath="{.data.password}" | base64 -d)
    
    # Login
    argocd login localhost:8080 --username admin --password $ARGOCD_PASSWORD --insecure
    
    # Kill port forward
    kill $PF_PID 2>/dev/null || true
    
    print_success "Logged into ArgoCD"
}

deploy_project() {
    print_header "Deploying Luminara Project"
    
    print_info "Applying ArgoCD project..."
    kubectl apply -f argocd/project.yaml
    
    print_success "Project deployed"
}

deploy_environment() {
    local ENV=$1
    
    print_header "Deploying to $ENV Environment"
    
    print_info "Applying application manifest..."
    kubectl apply -f "argocd/application-$ENV.yaml"
    
    if [ "$ENV" = "dev" ]; then
        print_info "Auto-sync is enabled for dev environment"
        print_info "Waiting for sync..."
        argocd app wait "$PROJECT_NAME-$ENV" --health --timeout 300
    else
        print_info "Manual sync required for $ENV environment"
        echo ""
        echo "To sync manually:"
        echo "  argocd app sync $PROJECT_NAME-$ENV"
    fi
    
    print_success "Application deployed to $ENV"
}

show_status() {
    local ENV=$1
    
    print_header "$ENV Environment Status"
    
    argocd app get "$PROJECT_NAME-$ENV"
    
    echo ""
    echo "View pods:"
    echo "  kubectl get pods -n luminara-$ENV"
}

show_logs() {
    local ENV=$1
    local SERVICE=$2
    
    print_header "Logs for $SERVICE in $ENV"
    
    kubectl logs -f "deployment/$SERVICE" -n "luminara-$ENV"
}

rollback() {
    local ENV=$1
    
    print_header "Rolling back $ENV Environment"
    
    print_info "Showing sync history..."
    argocd app history "$PROJECT_NAME-$ENV"
    
    echo ""
    read -p "Enter the revision ID to rollback to: " REVISION
    
    print_info "Rolling back to revision $REVISION..."
    argocd app rollback "$PROJECT_NAME-$ENV" "$REVISION"
    
    print_success "Rollback initiated"
}

delete_environment() {
    local ENV=$1
    
    print_header "Deleting $ENV Environment"
    
    read -p "Are you sure you want to delete $ENV environment? (yes/no): " CONFIRM
    
    if [ "$CONFIRM" = "yes" ]; then
        print_info "Deleting application..."
        argocd app delete "$PROJECT_NAME-$ENV" --cascade
        
        print_success "Environment deleted"
    else
        print_info "Deletion cancelled"
    fi
}

# Main menu
show_menu() {
    echo ""
    echo "Luminara GitOps Deployment Tool"
    echo "================================"
    echo "1. Check Prerequisites"
    echo "2. Install ArgoCD"
    echo "3. Deploy Luminara Project"
    echo "4. Deploy Development Environment"
    echo "5. Deploy Staging Environment"
    echo "6. Deploy Production Environment"
    echo "7. Show Environment Status"
    echo "8. View Logs"
    echo "9. Rollback Environment"
    echo "10. Delete Environment"
    echo "11. Exit"
    echo ""
}

main() {
    while true; do
        show_menu
        read -p "Select an option: " choice
        
        case $choice in
            1) check_prerequisites ;;
            2) install_argocd ;;
            3) deploy_project ;;
            4) deploy_environment "dev" ;;
            5) deploy_environment "staging" ;;
            6) deploy_environment "production" ;;
            7)
                read -p "Environment (dev/staging/production): " env
                show_status "$env"
                ;;
            8)
                read -p "Environment (dev/staging/production): " env
                read -p "Service name: " service
                show_logs "$env" "$service"
                ;;
            9)
                read -p "Environment (dev/staging/production): " env
                rollback "$env"
                ;;
            10)
                read -p "Environment (dev/staging/production): " env
                delete_environment "$env"
                ;;
            11)
                echo "Goodbye!"
                exit 0
                ;;
            *)
                print_error "Invalid option"
                ;;
        esac
        
        echo ""
        read -p "Press Enter to continue..."
    done
}

# Run main menu if no arguments provided
if [ $# -eq 0 ]; then
    main
else
    # Command line mode
    case "$1" in
        install)
            check_prerequisites
            install_argocd
            login_argocd
            ;;
        deploy)
            if [ -z "$2" ]; then
                print_error "Please specify environment: dev, staging, or production"
                exit 1
            fi
            deploy_project
            deploy_environment "$2"
            ;;
        status)
            show_status "${2:-dev}"
            ;;
        logs)
            show_logs "${2:-dev}" "${3:-api-gateway}"
            ;;
        rollback)
            rollback "${2:-dev}"
            ;;
        *)
            echo "Usage: $0 [install|deploy|status|logs|rollback] [environment] [service]"
            exit 1
            ;;
    esac
fi

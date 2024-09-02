# Argo CD

- [argocd](https://argo-cd.readthedocs.io/en/stable/)

## Install

### Create a namespace

- argocd: [install.yaml](https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml)
- [./custom.install.yaml](argocd/custom.install.yaml): insecure `argocd-server`

```bash
kubectl create namespace argocd

namespace/argocd created
```

### Install Argo CD

```bash
kubectl apply -n argocd -f argocd/custom.install.yaml
kubectl apply -n argocd -f argocd/ingressroute.yaml
```

### Install CLI

- argocd: [CLI Install](https://argo-cd.readthedocs.io/en/stable/cli_installation/)

```bash
curl -sSL -o argocd-linux-amd64 https://github.com/argoproj/argo-cd/releases/latest/download/argocd-linux-amd64
sudo install -m 555 argocd-linux-amd64 /usr/local/bin/argocd
rm argocd-linux-amd64
```

```bash
argocd --insecure --server argocd.localhost version
```

### Set Password

```bash
argocd --insecure --server argocd.localhost version
```

```bash
argocd login argocd.localhost

WARNING: server certificate had error: tls: failed to verify certificate: x509: certificate is valid for 8ba895139388b2114840eecf0244e4d9.71c94757b52b67b840cb71f51d4b2ff5.traefik.default, not argocd.localhost. Proceed insecurely (y/n)? y
Username: admin
Password:
'admin:login' logged in successfully
Context 'argocd.localhost' updated
```

```bash
argocd account update-password

argocd account update-password
*** Enter password of currently logged in user (admin):
*** Enter new password for user admin: argocdadmin
*** Confirm new password for user admin: argocdadmin
Password updated
Context 'argocd.localhost' updated
```

### Access the Argo CD API server

Open: [https://argocd.localhost/](https://argocd.localhost/)

![argocd](/images/argocd.png)

### Uninstall Argo CD

```bash
kubectl delete -n argocd -f argocd/ingressroute.yaml
kubectl delete -n argocd -f argocd/custom.install.yaml
```


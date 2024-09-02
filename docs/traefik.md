# Traefik

- [traefik](https://traefik.io/)
  - [install](https://doc.traefik.io/traefik/getting-started/install-traefik/)

## Install

### with Helm Chart

```bash
helm repo add traefik https://traefik.github.io/charts
```

```bash
helm repo ls
helm repo update
```

### Create a namespace

```bash
kubectl create ns traefik-v2

namespace/traefik-v2 created
```

### Install Traefik

```bash
helm install --namespace=traefik-v2 \
    --set="additionalArguments={--log.level=DEBUG}" \
    traefik traefik/traefik
```

```bash
helm list -n traefik-v2

NAME    NAMESPACE       REVISION        UPDATED                                 STATUS          CHART           APP VERSION
traefik traefik-v2      1               2024-09-02 22:53:07.715193723 +0900 KST deployed        traefik-30.1.0  v3.1.2
```

### Access the Traefik Dashboard

```bash
kubectl apply -f traefik/dashboard.yaml

ingressroute.traefik.io/dashboard created
```

Open: [http://traefik.localhost/dashboard/#/](http://traefik.localhost/dashboard/#/)


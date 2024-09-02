# Docker

- hub: [registry](https://hub.docker.com/_/registry)

## Start a registry

```bash
docker run -d -p 5000:5000 --restart unless-stopped --name registry registry:2
```

```bash
docker logs registry
```

## Push an image

```bash
cd backend
docker build -t localhost:5000/rurumimic/rust-on-kube/backend:0.0.1 .
docker push localhost:5000/rurumimic/rust-on-kube/backend:0.0.1
```


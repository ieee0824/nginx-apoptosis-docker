# nginx-apoptosis-docker

This is a Docker image that runs nginx. However, it will die in 24 hours.

## build docker image

```
$ docker build -t nginx-apoptosis-docker .
```

## run

```
$ docker run -d -v /host/path/nginx.conf:/etc/nginx/nginx.conf:ro --name foo-nginx nginx-apoptosis-docker
```

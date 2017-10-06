FROM rust:latest AS build

WORKDIR /usr/src/nginx/apoptosis-docker

COPY . .

RUN set -e \
	&& cargo install \
	&& cargo build --release

FROM nginx:latest 

COPY --from=build /usr/src/nginx/apoptosis-docker/target/release/nginx-apoptosis-docker /bin/nginx-apoptosis-docker

CMD ["nginx-apoptosis-docker"]

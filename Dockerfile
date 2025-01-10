FROM rust:1.74-bullseye as cargo
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim as rt
RUN apt-get update
RUN apt-get install -y --no-install-recommends ca-certificates
COPY --from=cargo /usr/local/cargo/bin/mdmd /usr/local/bin/mdmd
ENV TZ="Europe/London"
CMD ["mdmd"]

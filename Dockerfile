FROM docker.leryn.top/rust:1.60-slim-buster as build

WORKDIR /usr/local/app

COPY .cargo  ~
COPY .       .

RUN  cargo build --release \
     \
     bash build/build.sh

FROM docker.leryn.top/debian:buster-slim as publish

WORKDIR /opt

COPY --from=publsh /usr/local/app/release/* /usr/lib/vsp

CMD tail -f /dev/null
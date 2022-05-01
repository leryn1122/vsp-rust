FROM docker.leryn.top/rust:1.60-slim-buster as build

WORKDIR /opt/app

COPY .cargo  ~

COPY .       .

RUN  bash build/build.sh

FROM docker.leryn.top/debian:buster-slim as publish

WORKDIR  /opt

COPY --from=build /opt/app/release/* /usr/lib/vsp

COPY ./build/docker-entrypoint.sh /usr/local/bin/

RUN  ln -sf /usr/local/bin/docker-entrypoint.sh /entrypoint.sh

ENTRYPOINT ["docker-entrypoint.sh"]

CMD tail -f /dev/null
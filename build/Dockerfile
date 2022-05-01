FROM docker.leryn.top/rust:1.60-slim-buster as build

WORKDIR /opt/app

COPY .cargo  ~

COPY .       .

RUN cargo build --release ; \
    \
    mkdir -p release/{bin,conf,include,lib} ; \
    find target/release/ -maxdepth 1 -type f -perm -750 -exec cp -ar {} release/bin \; ; \
    awk -F ',' '{printf "cp -ar %s %s\n", $1, $2}' build/conf/list.csv ; \
    \
    apt update && apt install -y \
        gnupg \
    ; \
    { \
      echo "" ; \
      echo "deb https://mirrors.ustc.edu.cn/ubuntu/ focal main restricted universe multiverse" ; \
      echo "deb https://mirrors.ustc.edu.cn/ubuntu/ focal-security main restricted universe multiverse" ; \
      echo "deb https://mirrors.ustc.edu.cn/ubuntu/ focal-updates main restricted universe multiverse" ; \
      echo "deb https://mirrors.ustc.edu.cn/ubuntu/ focal-backports main restricted universe multiverse" ; \
    } >> /etc/apt/sources.list ; \
    apt-key adv --recv-keys --keyserver keyserver.ubuntu.com 8D81803C0EBFCD88 ; \
    apt-key adv --recv-keys --keyserver keyserver.ubuntu.com 7EA0A9C3F273FCD8 ; \
    apt-key adv --recv-keys --keyserver keyserver.ubuntu.com 3B4FE6ACC0B21F32 ; \
    apt update && apt install -y \
        upx \
    ; \
    find release/bin/ -maxdepth 1 -type f -exec upx {} \; ; \
    \
    echo "Compile finished." ;

FROM docker.leryn.top/debian:buster-slim as publish

WORKDIR  /opt

COPY --from=build /opt/app/release/ /usr/lib/vsp/

COPY ./build/docker-entrypoint.sh /usr/local/bin/

RUN  ln -sf /usr/local/bin/docker-entrypoint.sh /entrypoint.sh

ENTRYPOINT ["docker-entrypoint.sh"]

CMD tail -f /dev/null
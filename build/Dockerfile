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

COPY --from=build /opt/app/release/ /usr/lib/vsp/vsp-0.0.1/

COPY ./build/docker-entrypoint.sh /usr/local/bin/

RUN  ln -sf /usr/lib/vsp/vsp-0.0.1                 /usr/lib/vsp/default-vsp   ; \
     ln -sf /usr/lib/vsp/default-vsp/bin/vspc      /etc/alternatives/vspc     ; \
     ln -sf /usr/lib/vsp/default-vsp/bin/vspr      /etc/alternatives/vspr     ; \
     ln -sf /usr/lib/vsp/default-vsp/bin/vsps      /etc/alternatives/vsps     ; \
     ln -sf /usr/lib/vsp/default-vsp/bin/vspstk    /etc/alternatives/vspstk   ; \
     ln -sf /usr/lib/vsp/default-vsp/bin/vspx      /etc/alternatives/vspx     ; \
     ln -sf /etc/alternatives/vspc                 /usr/bin/vspc              ; \
     ln -sf /etc/alternatives/vspr                 /usr/bin/vspr              ; \
     ln -sf /etc/alternatives/vsps                 /usr/bin/vsps              ; \
     ln -sf /etc/alternatives/vspstk               /usr/bin/vspstk            ; \
     ln -sf /etc/alternatives/vspx                 /usr/bin/vspx              ; \
     ln -sf /usr/local/bin/docker-entrypoint.sh /entrypoint.sh

ENTRYPOINT ["docker-entrypoint.sh"]

CMD tail -f /dev/null
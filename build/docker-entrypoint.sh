#!/usr/bin/env bash

ln -sf /usr/lib/vsp/vsp-0.0.1                 /usr/lib/vsp/default-vsp   ; \
ln -sf /usr/lib/vsp/default-vsp/bin/vspc      /etc/alternatives/vspc     ; \
ln -sf /usr/lib/vsp/default-vsp/bin/vspr      /etc/alternatives/vspr     ; \
ln -sf /usr/lib/vsp/default-vsp/bin/vsps      /etc/alternatives/vsps     ; \
ln -sf /usr/lib/vsp/default-vsp/bin/vspstk    /etc/alternatives/vspstk   ; \
ln -sf /usr/lib/vsp/default-vsp/bin/vspx      /etc/alternatives/vspx     ; \
ln -sf /etc/alternatives/vspc                 /usr/bin/vspc              ; \
ln -sf /etc/alternatives/vspr                 /usr/bin/vspr              ; \
ln -sf /etc/alternatives/vsps                 /usr/bin/vsps              ; \
ln -sf /etc/alternatives/vspstk               /usr/bin/vspstk            ; \
ln -sf /etc/alternatives/vspx                 /usr/bin/vspx              ;
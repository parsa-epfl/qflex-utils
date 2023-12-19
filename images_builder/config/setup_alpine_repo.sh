#!/usr/bin/env sh

set -xe

cat > /etc/apk/repositories << EOF; $(echo)
https://ftp.halifax.rwth-aachen.de/alpine/v$(cut -d'.' -f1,2 /etc/alpine-release)/main/
https://ftp.halifax.rwth-aachen.de/alpine/v$(cut -d'.' -f1,2 /etc/alpine-release)/community/
https://ftp.halifax.rwth-aachen.de/alpine/edge/testing/
EOF



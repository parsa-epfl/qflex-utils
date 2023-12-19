#!/usr/bin/env sh

set -xe

# ──────────────────────────────────────────────────────────────────────────────

setup-keymap gb gb

# ─── Install SSH ──────────────────────────────────────────────────────────────

mkdir -p ~/.ssh
cp ./authorized_keys ~/.ssh/authorized_keys

# ──────────────────────────────────────────────────────────────────────────────

apk update
apk upgrade

# ─── Install Docker ───────────────────────────────────────────────────────────

apk add docker
rc-update add docker default

sleep 5s

service docker start

sleep 30s

service docker status



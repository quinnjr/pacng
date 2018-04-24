#!/usr/bin/sh

echo ':: Installing dependencies...'
pacman -Syu --noconfirm rustup openssl openssh curl sudo \
  ccache clang
rustup install nightly
rustup default nightly

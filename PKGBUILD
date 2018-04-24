# Maintainer: Joseph R. Quinn <quinn dot josephr at protonmail dot com>
pkgname="pacng"
pkgver="0.1.0"
pkgrel=1
pkgdesc="A high-performance pacman alternative with AUR support."
arch=("any")
url="https://github.com/quinnjr/pacng"
license=("MIT")
depends=("openssl" "git" "curl")
makedepends=("rust" "cargo" "llvm")
provides=("pacng")
conflicts=("pacng-git" "pacng-bin")
backup=("etc/pacng.toml")
install=".install"
changelog="https://github.com/quinnjr/pacng/wiki/Change-Log"
source=("$url/$pkgname-$pkgver.tar.gz")
sha256sums=()

prepare() {
  cd "$pkgname-$pkgver"
  cargo clean
}

build() {
	cd "$pkgname-$pkgver"
  cargo build --release --bin pacng
}

package() {
	cd "$pkgname-$pkgver"
  install -d 755 "$pkgdir/var/cache/pacng"
  install -d 755 "$pkgdir/etc/pacng.d"
  install -D 644 './etc/pacng.toml' "$pkgdir/etc/pacng.toml"
  install -D 644 'scripts/usr-lib-systemd-system-pacng-mirrorlist-update.timer' '$pkgdir/usr/lib/systemd/pacng-mirrorlist-update.timer'
  install -D 644 'scripts/usr-lib-systemd-system-pacng-mirrorlist-update.service' '$pkgdir/usr/lib/systemd/pacng-mirrorlist-update.service'
  cargo install --root "$pkgdir/usr"
}

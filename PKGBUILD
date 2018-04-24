# Maintainer: Joseph R. Quinn <quinn dot josephr at gmail dot com>
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
changelog="CHANGES.md"
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

check() {
	cd "$pkgname-$pkgver"
	cargo test
}

package() {
	cd "$pkgname-$pkgver"
  mkdir -p "$pkgdir/usr"
  mkdir -p "$pkgdir/var/cache/pacng"
  mkdir -p "$pkgdir/etc/pacng.d"
  cp './etc/pacng.toml' "$pkgdir/etc/pacng.toml"
  cargo install --root "$pkgdir/usr"
}

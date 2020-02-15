# Maintainer: Alejandro Valdes <alejandrovaldes at live dot com>

pkgname=gocd-encrypt
pkgver=0.1.0
pkgrel=1
pkgdesc="CLI to encrypt Strings using a remote gocd server"
arch=('x86_64')
url="https://github.com/alevalv/gocd-encrypt"
license=('GPLv2')
makedepends=('rust' 'cargo')
source=("https://github.com/alevalv/gocd-encrypt/archive/${pkgver}.zip")
sha512sums=('91b3e5003bbc2fa65d4c6f67697071fbb8eaced77e3be274482db92d8958a04837c378a4dc533948a9d1188407571b60e175dfb16212c4e0d61dd656d28d71d2')

prepare() {
  cd "${srcdir}/${pkgname}-${pkgver}"
  cargo fetch --locked
}

build() {
  cd "${srcdir}/${pkgname}-${pkgver}"
  cargo build --release --locked
}

check() {
  cd "${srcdir}/${pkgname}-${pkgver}"
  cargo test --release --locked
}

package() {
  cd "${srcdir}/${pkgname}-${pkgver}"
  install -Dm 755 "target/release/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"
}

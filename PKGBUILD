# Maintainer: Logan Koester <logan@logankoester.com>
pkgname=smaug
pkgver=0.2.0
pkgrel=1
pkgdesc="A tool to manage your DragonRuby Game Toolkit projects"
arch=('x86_64')
url="https://smaug.dev/"
license=('AGPL3')
depends=(gcc-libs openssl zlib)
optdepends=()
source=("https://github.com/ereborstudios/smaug/releases/download/v${pkgver}/smaug-linux")
noextract=()
sha512sums=('ed0d7171ba03b375a525498e5c4b1b3cfc8d30abfe58369ee8120b71de434d2f9c5a13eb6397fe4f70c5c89dbc45d1a4e916321223c8958483b393a1ec2a77cb')

package() {
  cd $srcdir
  install -Dm755 "${pkgname}-linux" "${pkgdir}/usr/bin/${pkgname}"
}

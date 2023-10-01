# Maintainer: Matthias Gabriel <gabm+aur@mailbox.org>
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=satty-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="A sreenshot annotation tool inspired by Swappy and Flameshot"
url="https://github.com/gabm/satty"
license=("MPL-2.0")
arch=("x86_64")
provides=("satty")
conflicts=("satty")
source=("https://github.com/gabm/satty/releases/download/v$pkgver/satty-$pkgver-x86_64.tar.gz")
sha256sums=("89f18060df8b28bf48c1811396e82b4c04457a869e36c9170d82ed4d69d00337")

package() {
    install -Dm755 satty -t "$pkgdir/usr/bin"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

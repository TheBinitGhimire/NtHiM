# Maintainer: Nabeen Tiwaree <iym.nabeen@gmail.com>

pkgname=NtHiM
pkgver=2.0.0
pkgrel=1
zipfile=x86_64-${pkgname}-linux-gnu.zip
pkgdesc="Super Fast Sub-domain Takeover Detection tool written in rust by late @TheBinitGhimire"
arch=("x86_64")
url="https://github.com/TheBinitGhimire/NtHiM"
license=("MIT")
makedepends=("unzip")
source=("https://github.com/TheBinitGhimire/${pkgname}/releases/download/0.1.4/${zipfile}")
b2sums=('ee280fd9dedeb9ed4d9be8f71688b80507a798fe46d4dc20f6b4405ac33d2d3eb07bae356139fade7ecc6efec802a55cf2c6bf9c9b9de8a838ad5fe3364c6b7d')
build() {
  unzip ${zipfile}
}

package() {
    install -Dm755 ${pkgname} ${pkgdir}/usr/bin/${pkgname}
}

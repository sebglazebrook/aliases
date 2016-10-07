#!/bin/bash -u

echo "Building package for version number ${VERSION_NUMBER}"

PKG_DIR="${WORK_DIR}/${PKG_NAME}_${VERSION_NUMBER}"

cd ${WORK_DIR}

mkdir ${PKG_DIR} && cd ${PKG_DIR} \
                 && mkdir -p usr/local/bin \
                 && mkdir -p DEBIAN \
                 && cat ${CODE_DIR}/packagers/debian/templates/DEBIAN/control | sed "s/<<VERSION_NUMBER>>/${VERSION_NUMBER}/" > DEBIAN/control

cd ${CODE_DIR} && cargo build --release

cp ${CODE_DIR}/target/release/aliases ${PKG_DIR}/usr/local/bin/aliases

cd ${WORK_DIR} && dpkg-deb --build ${PKG_NAME}_${VERSION_NUMBER}

ls ${PKG_NAME}_${VERSION_NUMBER}.deb

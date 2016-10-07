#!/bin/bash -u

VERSION_NUMBER=${1}

docker build -t alias-debian-pkg-builder -f packagers/debian/Dockerfile .

CONTAINER_ID=$(docker create -e VERSION_NUMBER="$VERSION_NUMBER" alias-debian-pkg-builder)

docker start -ai ${CONTAINER_ID}

docker cp ${CONTAINER_ID}:/tmp/aliases_${VERSION_NUMBER}.deb .

mkdir -p releases/$VERSION_NUMBER/debian && mv aliases_${VERSION_NUMBER}.deb releases/$VERSION_NUMBER/debian/

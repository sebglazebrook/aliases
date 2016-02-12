#!/bin/bash

targz() {
  local tmpFile="${@%/}.tar"
  tar -cvf "${tmpFile}" --exclude=".DS_Store" "${@}" || return 1
  size=$(
  stat -f"%z" "${tmpFile}" 2> /dev/null; # OS X `stat`
  stat -c"%s" "${tmpFile}" 2> /dev/null # GNU `stat`
  )
  local cmd=""
  if (( size < 52428800 )) && hash zopfli 2> /dev/null; then
    # the .tar file is smaller than 50 MB and Zopfli is available; use it
    cmd="zopfli"
  else
    if hash pigz 2> /dev/null; then
      cmd="pigz"
    else
      cmd="gzip"
    fi
  fi
  echo "Compressing .tar using \`${cmd}\`…"
  "${cmd}" -v "${tmpFile}" || return 1
  [ -f "${tmpFile}" ] && rm "${tmpFile}"
  echo "${tmpFile}.gz created successfully."
}

VERSION_NUMBER="$1"

if [ `basename "${PWD}" != "aliases"` ]; then
  echo "Looks like you're not in the app root, can't continue."
  exit 1
fi

# update the version number here?

cargo build --release

cd target/release

targz aliases

cd ../..

mkdir -p releases/$VERSION_NUMBER/mac && mv target/release/aliases.tar.gz releases/$VERSION_NUMBER/mac/

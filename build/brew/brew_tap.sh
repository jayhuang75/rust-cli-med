#!/bin/bash
echo ">>> download package"
pwd
curl https://github.com/jayhuang75/rust-cli-med/releases/download/0.6.2/macos_x86_archive-0.6.2.tar.gz > macos_x86_archive-0.6.2.tar.gz
ls -la

echo ">>> shasum"
SHASUM=$(shasum -a 256 macos_x86_archive-0.6.2.tar.gz | awk '{printf $1}')

echo ">>> brew tap"
brew tap jayhuang75/med

echo ">>> brew bump formula pr"
echo ${SHASUM}
brew bump-formula-pr -v --version=0.6.2 --no-browse --no-audit --sha256="${SHASUM}" --url="https://github.com/jayhuang75/rust-cli-med/releases/download/0.6.2/macos_x86_archive-0.6.2.tar.gz" jayhuang75/med/med
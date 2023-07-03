#!/bin/bash
echo ">>> install tools needed "
sudo dnf install fedora-packager rpmdevtools tree wget gcc copr-cli -y -q

echo ">>> create build structure"
rpmdev-setuptree

echo ">>> move spec file"
pwd
cp ./build/med.spec ~/rpmbuild/SPECS

echo ">>> wget source file"
spectool -gR ./build/med.spec -g

echo ">>> build the spec"
cd ~/rpmbuild
rpmbuild -ba ./SPECS/med.spec
tree

echo ">>> update fedora corp"
copr-cli --config /__w/rust-cli-med/rust-cli-med/build/fedora_conf build med ./SRPMS/med-*.src.rpm 
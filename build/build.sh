#!/bin/bash
echo ">>> install tools needed "
sudo dnf install fedora-packager rpmdevtools tree wget gcc copr-cli -y -q

echo ">>> create build structure"
rpmdev-setuptree

echo ">>> move spec file"
cp ./build/med.spec ~/rpmbuild/SPECS

echo ">>> wget source file"
spectool -gR ./build/med.spec -g

echo ">>> build the spec"
cd ~/rpmbuild
rpmbuild -ba ./SPECS/med.spec
tree

echo ">>> write to ~/.config/copr"
echo $FEDORA_COPR >> ~/.config/copr

echo ">>> update fedora corp"
copr-cli build med ./SRPMS/med-*.src.rpm 
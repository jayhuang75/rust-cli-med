#!/bin/bash

RPM_BUILD_SOURCES="$HOME/rpmbuild/SOURCES"
RPM_BUILD_SPECS="$HOME/rpmbuild/SPECS"
RPM_BUILD="$HOME/rpmbuild/BUILD"
BUILD_SPECS_LOCATION="$HOME/rust/rust-cli-masker/build/med.spec"

RELEASES_PACKAGE_URL="https://github.com/jayhuang75/rust-cli-med/releases/download"
RELEASES_PACKAGE_VERSION="0.6.0"
RELEASES_PACKAGE_NAME="test-med-x86-unknow-linux-gnu-0.6.0.tar.gz"

echo creat structure
mkdir -p $RPM_BUILD_SOURCES $RPM_BUILD_SPECS $RPM_BUILD

echo copy specs to build specs
cp $BUILD_SPECS_LOCATION $RPM_BUILD_SPECS

echo download release packages
cd $RPM_BUILD_SOURCES
wget -N $RELEASES_PACKAGE_URL/$RELEASES_PACKAGE_VERSION/$RELEASES_PACKAGE_NAME -q
ls -la
pwd

echo tar release packages
tar -xf $RELEASES_PACKAGE_NAME -C $RPM_BUILD
ls -la

echo copy med bin to the build root
cp $RELEASES_PACKAGE_NAME $RPM_BUILD_SOURCES

echo copy package files
cp $RPM_BUILD/med-0.6.0/med $HOME/rpmbuild

echo build the rpm
cd $HOME/rpmbuild
rpmbuild -ba --build-in-place SPECS/med.spec

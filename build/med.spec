Name:           med
Version:        0.6.1
Release:        1%{?dist}
Summary:        A simple enough CLI tool with auditable capability for Data Masking/Encyption/Decryption for CSV/JSON files.

License:        Apache License 2.0
URL:            https://github.com/jayhuang75/rust-cli-med
Source0:        https://github.com/jayhuang75/rust-cli-med/releases/download/%{version}/med-x86-unknow-linux-gnu-%{version}.tar.gz

Requires:       bash

%description
A simple enough CLI tool with auditable capability for Data Masking/Encyption/Decryption for CSV/JSON files.

%global debug_package %{nil}
%prep
%setup -q

%build

%install
rm -rf $RPM_BUILD_ROOT
mkdir -p $RPM_BUILD_ROOT/%{_bindir}
cp %{name} $RPM_BUILD_ROOT/%{_bindir}

%clean
rm -rf $RPM_BUILD_ROOT

%files
%{_bindir}/%{name}

%changelog
* Fri Jun 30 2023 jayhuang75 <jayhuang75@gmail.com>
- Rust powered performance.
- Provide Masking, and Encyption/Decryption capabilities.
- Auditable with build-in SQLite powered Audit table.

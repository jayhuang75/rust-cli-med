.PHONY:
update_fedora:
	sudo dnf update && sudo dnf upgrade && sudo dnf autoremove

.PHONY:
update_debian:
	sudo apt update -y && sudo apt upgrade -y && sudo apt autoremove -y && sudo apt autoclean -y

.PHONY:
json_encrypt:
	cargo run --bin med encrypt -t json -f demo/data/input/json -c demo/conf/conf_json.yaml -w 6 -k 1q2w3er -s des64

.PHONY:
json_mask:
	cargo run --bin med mask -t json -f demo/data/input/json -c demo/conf/conf_json.yaml -w 6

.PHONY:
csv_mask:
	cargo run --bin med mask -f demo/data/input/csv -c demo/conf/conf_csv.yaml -w 6

.PHONY:
csv_encrypt:
	cargo run --bin med encrypt -f demo/data/input/csv -c demo/conf/conf_csv.yaml -w 6 -k 1q2w3e4r -s des64

.PHONY:
csv_mask_performance:
	cargo run --bin med mask -f /Users/huangwh/rust/rust-design-pattern/demo -c demo/conf/conf_csv.yaml -w 6

.PHONY:
test_package:
	cargo build --release
	cp -R demo target/release
	mkdir target/release/med-0.6.0
	mv target/release/med target/release/med-0.6.0
	tar --directory=target/release -cf macos_x86_archive-test.tar.gz med-0.6.0 demo

.PHONY:
pre_release:
	cargo fmt
	cargo clippy
	cargo tarpaulin
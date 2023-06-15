.PHONY:
update_fedora:
	sudo dnf update && sudo dnf upgrade && sudo dnf autoremove

.PHONY:
update_debian:
	sudo apt update -y && sudo apt upgrade -y && sudo apt autoremove -y && sudo apt autoclean -y

.PHONY:
json_encrypt:
	cargo run --bin med encrypt -t json -f demo/data/input/json -c demo/conf/conf_json.yaml -w 12 -k 1q2w3er -s des64

.PHONY:
json_mask:
	cargo run --bin med mask -t json -f demo/data/input/json -c demo/conf/conf_json.yaml -w 12

.PHONY:
csv_mask:
	cargo run --bin med mask -f demo/data/input/csv -c demo/conf/conf_csv.yaml -w 12

.PHONY:
csv_encrypt:
	cargo run --bin med encrypt -f demo/data/input/csv -c demo/conf/conf_csv.yaml -w 12 -k 1q2w3e4r -s des64



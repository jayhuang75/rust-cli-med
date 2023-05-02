.PHONY:
update_fedora:
	sudo dnf update && sudo dnf upgrade && sudo dnf autoremove

.PHONY:
update_debian:
	sudo apt update -y && sudo apt upgrade -y && sudo apt autoremove -y && sudo apt autoclean -y

.PHONY:
run:
	RUST_LOG=info cargo run -q --release| bunyan

.PHONY:
db_migration:
	sqlx database create
	sqlx migrate run

.PHONY:
db_reset:
	sqlx database drop
	sqlx database create
	sqlx migrate run


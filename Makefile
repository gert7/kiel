all:
	cargo build --release
	sudo ./install_units.sh

all:
	cargo build --release
	sudo ./install_units.sh 1

nostart:
	cargo build --release
	sudo ./install_units.sh 0

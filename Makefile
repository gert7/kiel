all:
	sudo docker container stop homeassistant
	cargo build --release
	sudo ./install_units.sh 1
	sudo docker container start homeassistant

nostart:
	cargo build --release
	sudo ./install_units.sh 0

onlystart:
	sudo ./install_units.sh 1
	sudo docker container start homeassistant


all:
	docker container stop homeassistant
	cargo build --release
	./install_units.sh 1
	docker container start homeassistant

nostart:
	cargo build --release
	./install_units.sh 0

onlystart:
	./install_units.sh 1
	docker container start homeassistant


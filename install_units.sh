#!/bin/sh

export USYSDIR=/usr/lib/systemd/system/

sudo systemctl stop kiel.target
sudo systemctl stop kielserver.service
sudo systemctl stop kieltimer.timer
sudo systemctl stop kieltimer.service
sudo systemctl stop kielfetch.timer
sudo systemctl stop kielfetch.service

cd $HOME
mkdir -p $USYSDIR
cd -
cp systemd_units/* $USYSDIR

sudo systemctl daemon-reload

sudo cp target/release/server /usr/local/bin/kielserver
sudo cp target/release/kiel /usr/local/bin/kiel

sudo mkdir -p /etc/kiel.d
sudo cp default.toml /etc/kiel.d/default.toml
sudo cp .env /etc/kiel.d/.env

echo "Units installed. Execute 'sudo systemctl start kiel.target' to start services."


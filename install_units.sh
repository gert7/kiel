#!/bin/sh

export USYSDIR=/usr/lib/systemd/system/

sudo systemctl stop kiel.target
sudo systemctl stop kielserver.service
sudo systemctl stop kieltimer.timer
sudo systemctl stop kieltimer.service
sudo systemctl stop kielfetch.timer
sudo systemctl stop kielfetch.service
sudo systemctl stop homeguarantee.timer
sudo systemctl stop homeguarantee.service

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

if [ $1 == 0 ]; then
  echo "Units installed. Execute 'sudo systemctl start kiel.target' to start services."
else
  echo "Units installed."
  sudo systemctl start kiel.target
  sudo systemctl start homeguarantee.service
fi


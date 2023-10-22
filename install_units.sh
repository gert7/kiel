#!/bin/sh

export USYSDIR=/usr/lib/systemd/system/

systemctl stop kiel.target
systemctl stop kielserver.service
systemctl stop kieltimer.timer
systemctl stop kieltimer.service
systemctl stop kielfetch.timer
systemctl stop kielfetch.service
systemctl stop homeguarantee.timer
systemctl stop homeguarantee.service

cd "$HOME" || exit
mkdir -p $USYSDIR
cd - || exit
cp systemd_units/* $USYSDIR

systemctl daemon-reload

cp target/release/server /usr/local/bin/kielserver
cp target/release/kiel /usr/local/bin/kiel
# cp phue/boiler.py

mkdir -p /etc/kiel.d
cp default.toml /etc/kiel.d/default.toml
cp .env /etc/kiel.d/.env

if [ "$1" = 0 ]
then
  echo "Units installed. Execute 'sudo systemctl start kiel.target' to start services."
else
  echo "Units installed."
  systemctl enable kiel.target
  systemctl start kiel.target
  systemctl enable homeguarantee.service
  systemctl start homeguarantee.service
fi

if [ "$2" = "nofetch" ]
then
  systemctl stop kielfetch.timer
  systemctl stop kielfetch.service
fi


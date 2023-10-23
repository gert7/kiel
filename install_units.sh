#!/bin/sh

if [ "$1" = "" ]; then
  echo
  echo "Usage: ./install_units.sh START [--nofetch]"
  echo
  echo "Specify the options for installing units:"
  echo
  echo "    START - 0   Install units without starting them"
  echo "            1   Install units and start"
  echo ""
  echo "    --nofetch   Configure kiel not to fetch pricing data"
  echo "                from Nord Pool."
  exit
fi

export USYSDIR=/usr/lib/systemd/system/

systemctl stop kiel.target
systemctl stop kielserver.service
systemctl stop kieltimer.timer
systemctl stop kieltimer.service
systemctl stop kielfetch.timer
systemctl stop kielfetch.service

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
fi

if [ "$2" = "--nofetch" ]
then
  systemctl stop kielfetch.timer
  systemctl disable kielfetch.timer
else
  systemctl start kielfetch.timer
fi


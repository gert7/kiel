#!/bin/sh

export USYSDIR=/usr/lib/systemd/system/

cd $HOME
mkdir -p $USYSDIR
cd -
cp systemd_units/* $USYSDIR

sudo cp target/release/server /usr/local/bin/kielserver

<<<<<<< HEAD
sudo cp target/debug/server /usr/local/bin/kielserver

sudo mkdir -p /etc/kiel.d
sudo cp default.toml /etc/kiel.d/default.toml
sudo cp .env /etc/kiel.d/.env
=======
>>>>>>> 808c09b1f03f70a2ea9b8c3a1c58d670bb16b6ba

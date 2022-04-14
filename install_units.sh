#!/bin/sh

export USYSDIR=/usr/lib/systemd/system/

cd $HOME
mkdir -p $USYSDIR
cd -
cp systemd_units/* $USYSDIR

sudo cp target/release/server /usr/local/bin/kielserver
sudo cp target/release/kiel /usr/local/bin/kiel


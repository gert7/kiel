#!/bin/sh

export USYSDIR=.config/systemd/user

cd $HOME
mkdir -p $USYSDIR
cd -
cp systemd_units/kielserver.service $HOME/$USYSDIR

sudo cp target/debug/server /usr/local/bin/kielserver

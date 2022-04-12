#!/usr/bin/python

import os
import sys
from phue import Bridge
from dotenv import load_dotenv

load_dotenv("../.env")

# 00:17:88:01:0b:df:92:fa-0b

def set_light(b: Bridge, i: int, state: bool):
    b.set_light(i, 'on', state)

def toggle_light(b: Bridge, i):
    is_on = b.get_light(i, 'on')

    if is_on:
        b.set_light(i, 'on', False)
    else:
        b.set_light(i, 'on', True)

def main():
    power_state = sys.argv[1]
    state: Optional[bool] = None
    if power_state == "0":
        state = False
    elif power_state == "1":
        state = True
    else:
        print("No valid power argument supplied!")
        print("Valid arguments are: 0 = off, 1 = on")
        exit(1)

    plug_uid = os.getenv("KIEL_PLUG_UID")
    special_word = os.getenv("KIEL_SPECIAL_WORD").lower()

    b = Bridge('192.168.1.201')

    b.connect()
    api: dict = b.get_api()
    for key, light in api["lights"].items():
        if light['uniqueid'] == plug_uid:
            to_i = int(key)
            set_light(b, to_i, state)
            print(light['uniqueid'])
            continue
        if light['name'].lower().__contains__(special_word):
            to_i = int(key)
            set_light(b, to_i, state)
            print(light['uniqueid'])

if __name__ == "__main__":
    main()
#!/usr/bin/python

import sys
from phue import Bridge

state = sys.argv[1]

if state not in ["on", "off"]:
    print("invalid argument. must be 'on' or 'off'")

b = Bridge('192.168.1.201')

b.connect()

api = b.get_api()

print(api['lights']['1'])

if state == 'on':
    b.set_light(1, 'on', True)
else:
    b.set_light(1, 'on', False)

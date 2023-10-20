#!/bin/sh

docker buildx build -t kiel --platform linux/arm/v7 . --output=targetcontainer

mv targetcontainer/opt/kiel/target/release ./target_armv7

rm -rf targetcontainer


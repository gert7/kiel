#!/bin/sh

docker run --privileged --rm tonistiigi/binfmt --install all

echo "If buildx gets stuck for a long time exporting files, press CTRL-C once to continue."
echo "Be sure to check if the files were actually exported."

docker buildx build -t kiel --no-cache --platform linux/arm/v7 . --output=targetcontainer

rm -rf target_armv7

mv targetcontainer/opt/kiel/target/release ./target_armv7

rm -rf targetcontainer


sudo apt install libpq-dev postgresql libssl-dev docker.io
sudo docker run -d --name homeassistant --privileged --restart=unless-stopped -e TZ=Europe/Tallinn -v /home/pi/homeassistant:/config --network=host ghcr.io/home-assistant/home-assistant:stable


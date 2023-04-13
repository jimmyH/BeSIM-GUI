# BeSIM GUI

First stab at a basic Angular GUI for [BeSIM](https://github.com/jimmyH/BeSIM), so you can control the thermostat from your browser on a PC or Mobile Phone.

It isn't very pretty, I don't do GUIs...

Currently you need to run the `set_api_url.js` script to set the URL of your BeSIM server in the angular environment.

To run manually:
 - `API_URL=http://<your besim server>/api/v1.0/ node set_api_url.js
 - `ng serve --host 0.0.0.0 --disable-host-check`

To run from docker:
 - `docker build . -t besim_gui:latest`
 - `API_URL=http://<your besim server>/api/v1.0/ docker run -e API_URL -it -p 4200:4200 besim_gui:latest`

Then connect to port 4200 from a browser.

For a production environment:
 - `docker build -f Dockerfile.prod --build-arg API_URL=http://<your besim server>/api/v1.0/ . -t besim_gui:latest`
 - `docker run -p 80:80 besim_gui:latest`



FROM node:14-alpine
RUN npm install -g @angular/cli@11
EXPOSE 4200
COPY . /app
WORKDIR /app
RUN cd /app && npm install
CMD node set_api_url.js && ng serve --host 0.0.0.0 --disable-host-check

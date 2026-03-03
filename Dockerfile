# Run using: docker run -it --rm -e API_URL=https:///besim-api.kagisoft.co.uk/api/v1.0/ <...>
FROM node:22-alpine
RUN npm install -g @angular/cli@20
EXPOSE 4200
ENV NG_CLI_ANALYTICS=ci
ARG API_URL
COPY . /app
WORKDIR /app
RUN cd /app && npm install
CMD node set_api_url.js && ng serve --host 0.0.0.0 --disable-host-check

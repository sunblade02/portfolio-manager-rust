ARG NODE_IMAGE_TAG=24-alpine

FROM node:${NODE_IMAGE_TAG}

RUN apk add --no-cache chromium nss freetype harfbuzz ttf-freefont

ENV CHROME_BIN=/usr/bin/chromium-browser

RUN npm install -g @angular/cli

RUN npm config set strict-ssl false
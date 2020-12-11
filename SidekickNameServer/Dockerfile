FROM node:14

WORKDIR /var/opt/nameserver

ADD build build
ADD node_modules node_modules
ADD package.json ./

ENTRYPOINT node build/index.js
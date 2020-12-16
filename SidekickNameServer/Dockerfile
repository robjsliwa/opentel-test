FROM node:14

WORKDIR /var/opt/nameserver

COPY ["package.json", "package-lock.json*", "./"]

RUN npm install

COPY . .

RUN npm run build

ENTRYPOINT node build/index.js
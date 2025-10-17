FROM node:22

WORKDIR /app

COPY package.json .
COPY package-lock.json .

# `clean install`
RUN npm ci

COPY src/index.js index.js

EXPOSE 3000

CMD ["node", "Index.js"]

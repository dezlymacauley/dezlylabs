FROM node:25.0.0-alpine3.21

WORKDIR /app

COPY ./package.json ./
COPY package-lock.json ./

RUN npm ci

COPY ./ ./

CMD ["npm", "start"]

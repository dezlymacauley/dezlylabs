FROM node:25.0.0-alpine3.21

WORKDIR /app

COPY ./package.json ./
COPY package-lock.json ./

RUN npm ci

COPY ./ ./

ENV PORT=5000
ENV APP_NAME="Cool Express App"

CMD ["npm", "start"]

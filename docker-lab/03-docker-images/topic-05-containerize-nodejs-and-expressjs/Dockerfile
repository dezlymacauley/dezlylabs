# This will use version 23 of Node.js as the base image
FROM node:23

# Sets the working directory inside the container to /app
# All commands and paths will be relative to this directory
WORKDIR /app

# This will copy the `package.json` and `package-lock.json` files
# from your local directory, to the working directory of the container.
COPY package.json ./
COPY package-lock.json ./

# This will install dependencies using a clean install
RUN npm ci

# This will copy the source code to the working directory of container
# So `src/` of your local directory will be located in `/app/src/`
COPY src/ ./

# NOTE: Always copy your source code AFTER you have installed 
# your dependencies. 
# Docker builds an image using a layered (aka step by step approach).
# If you make changes to a line, Docker has to re-execute all 
# of the commands below it.

# So you don't want Docker to re-install dependencies any time you change 
# your source code.
 
# This will allow external access to the container via port 3000 
# of the container.
EXPOSE 3000

# This will run the command `src/node index.js` to start the server.
CMD ["node", "src/index.js"]

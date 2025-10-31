FROM nginx:1.27.0

RUN apt-get update
RUN apt-get -y install vim

# This will copy the `index.html` file from this directory into the
# location in the Docker container.
COPY index.html /usr/share/nginx/html/index.html

RUN chown nginx:nginx /usr/share/nginx/html/index.html

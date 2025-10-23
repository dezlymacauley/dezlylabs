# `RUN` is for commands that you want to be executed during creation of
# a Docker image. E.g. Installing project dependencies.
# E.g. `RUN bun install --exact` 

# `CMD` if for commands that you want to be executed from inside a container
# instance that has been created from your Docker image.
CMD [ "bun", "run", "dev" ]

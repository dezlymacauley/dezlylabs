Docker Compose is for multi-container project setups.

If you production deployment is a single host setup, 
then you can get away with using Docker Compose instead of something more
complex than Kubernetes or Docker swarm, 
which would be overkill for a single host setup.

This is very common when creating a microservice architecture.

It minimizes the potential for configuration errors, 
by ensuring that there is consistency accross environments.

It ensures that each container starts in the correct order.
E.g A backend container should only start once the containers that have
its caching and database are running.

Docker compose also makes it easy to manage communication 
between containers and volumes for persistent storage across containers.

Docker compose allows you to get a consolidated view 
of your application architecture.

Another advantage of Docker Compose, 
is that it allows you to replicate a production environment for testing
and staging.

Naturally this means that Docker Compose can be paired with CI/CD Pipelines.

This is the standalone docker composer

E.g.
```sh
docker-compose up -d
bash tests.sh
docker-compose down
```

NOTE:

This is the new syntax. This is the `compose` plugin
```sh
docker compose
```

What is a Docker Image?

A Docker Image is a blueprint that is used to create an instance 
of a container container.

It contains everything that you need to run application.

A Docker Image contains the following:
1. A base image: This is usually a minimal Linux distribution
2. The runtime environment: This is project-specific. E.g. Python, Node.js
3. Project dependencies: This is project specific. E.g. PyPi, npm packages
4. Application code: This is your source code or compiled binaries.
5. Configuration files: This would be things like environment variables.
_______________________________________________________________________________

A `Dockerfile` is a used to progragmatically create a Docker Image.

_______________________________________________________________________________


_______________________________________________________________________________
```sh
docker run -d \
-p 127.0.0.1:8080:80/tcp \
--name nginx_container \
nginx:1.29.1
```

_______________________________________________________________________________

To view the current logs:
```sh
docker logs nginx_container
```
_______________________________________________________________________________

To keep watching for logs the current logs:
```sh
docker logs -f nginx_container
```

_______________________________________________________________________________

How to run shell commands from inside the container, 
using the `bash` program inside the container:
```sh
docker exec -it nginx_container /bin/bash
```

Your shell prompt will now look like this:
```
root@806513b18d71:/#
```

NOTE: The reason why this Nginx container has the program `bash` 
is because many official images are built on top of a Linux image.

You can confirm this by running this command inside the prompt 
of the container:
```
cat /etc/os-release
```

As you can see this is Debian:
```
PRETTY_NAME="Debian GNU/Linux 12 (bookworm)"
NAME="Debian GNU/Linux"
VERSION_ID="12"
VERSION="12 (bookworm)"
VERSION_CODENAME=bookworm
ID=debian
HOME_URL="https://www.debian.org/"
SUPPORT_URL="https://www.debian.org/support"
BUG_REPORT_URL="https://bugs.debian.org/"
```

To exit the shell prompt of the container, type this command:
```
exit
```
___________________________________________________________________________

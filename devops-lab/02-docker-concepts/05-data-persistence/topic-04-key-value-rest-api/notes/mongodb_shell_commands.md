This commands must be run from the Mongo DB shell in a Docker container.

This will let you enter the container via an interactive Mongo DB shell
```sh
docker exec -it mongodb mongosh
```
_______________________________________________________________________________

Show databases
```
show dbs;
```

```
admin   40.00 KiB
config  12.00 KiB
local   40.00 KiB
```

_______________________________________________________________________________

To use a specific database:

```
use admin;
```

Your shell prompt will change to this:
```
admin>
```
_______________________________________________________________________________

To show all of the connections that are stored under the admin database,
do this:

```
show connections;
```

To exit:
```
exit
```

_______________________________________________________________________________

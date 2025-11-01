
This will make queries easier to read when using Postgres
```
\x
```


List all databases in the current Postgres cluster.
```
\l
```

A Postgre cluster is made up of the following:

1 Postgres Instance 
+ All of the databases in that instance 
+ And the data directory

You will see three databases:

postgres, template0, and template 1

Do NO delete them.

Check what database you are connected to and as which user
```sh
\conninfo
```

Display all the tables in the current database
```sh
\dt
```

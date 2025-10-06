# Final Project
_______________________________________________________________________________
### About 

Subscription service

You buy one of three subscriptions

Allow people to create accounts so that they can log in

- Backend Language:         Go
- Relational Database:      PostgreSQL
- Store for Session Info:   Redis
- Mail Server:              Mailhog
- Containerization:         Docker

_______________________________________________________________________________
### Project Structure

cmd

_______________________________________________________________________________
### External Dependencies

You can search for external packages here:

https://pkg.go.dev/

A low-level driver for PostgreSQL (raw protocol, minimal abstractions)
```sh
go get github.com/jackc/pgconn
```

A high-level PostgreSQL library. Wraps pgconn for easier use (connection pool, query helpers)
```sh
go get github.com/jackc/pgx/v4/stdlib
```

HTTP session management for Go (middleware to manage login sessions, cookies, etc.)
```sh
go get github.com/alexedwards/scs/v2
```

Redis-backed session store for scs (stores session data 
in Redis instead of memory or files)
```sh
go get github.com/alexedwards/scs/redisstore
```

Lightweight and idiomatic router for building HTTP services in Go
```sh
go get github.com/go-chi/chi/v5/middleware
```

NOTE: To remove a package, remove it from the `go.mod` file,
then run `go mod tidy`

_______________________________________________________________________________

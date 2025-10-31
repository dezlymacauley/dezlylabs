package db

import (

    // Go's standard database interface.
    // It provides a generic API for SQL-like databases,
    // but requires a driver to work with a specific database.
    "database/sql"

    // SQLite driver for database/sql.
    // The underscore means we import it for its side-effects 
    // (driver registration), but don’t call it directly.
    _"github.com/mattn/go-sqlite3"
)

// The variable name is Uppercase so that code written outside of this file
// can interact with the database.
var DB *sql.DB

// NOTE: 

// A function to initialize the database
func InitDB() {
    var err error

    // The first parameter is the driver name (The type of SQL 
    // database that you want to use)
    DB, err = sql.Open("sqlite3", "api.db")

    if err != nil {
        // This will make the app crash if it fails 
        // to establish a database connection.
        panic("Could not connect to the database")
    }

    // Check that the DB is actually reachable
    if err = DB.Ping(); err != nil {
        panic("Could not ping the database")
    }

    // Set the maximum number of connections you 
    // can have open for the database.

    // If more than 10 things try to connect to the database, 
    // they will have to wait for open connections
    DB.SetMaxOpenConns(10)

    // Sets the maximum unused open connections. 
    DB.SetMaxIdleConns(5)

    createTables()
}

func createTables() {
    createEventsTable := `
    CREATE TABLE IF NOT EXISTS events (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        description TEXT NOT NULL,
        location TEXT NOT NULL,
        dateTime DATETIME NOT NULL,
        user_id INTEGER
    )
    `
    _, err := DB.Exec(createEventsTable)

    if err != nil {
        panic("Could not create events table")
    }

}

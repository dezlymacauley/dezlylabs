package models

import "time"

type Event struct {
    // These are struct tags. They tell Go that these fields are required
    // when created an instance of Event.
    ID int
    Name string `binding:"required"`
    Description string `binding:"required"`
    Location string `binding:"required"`
    DateTime time.Time `binding:"required"`
    UserID int 
}

// This is a variable to store the list of events
// slice literal
// `events` is private because it is lowercase, so it is only accessible
// by code inside this `event.go` file
var events []Event = []Event{}

// This is a method that will be available to each instance of Event.
// It will save the the event to the database
func (e Event) Save() {
    // This will create a new underlying array,
    // that is made up of the events slice literal, plus the new event (e)
    events = append(events, e)
}

// This is an external function that files outside, event.go,
// can use to view events when they import this file.
func GetAllEvents() []Event {
    return events
}

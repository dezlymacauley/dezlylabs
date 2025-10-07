package models

import "time"

type Event struct {
    ID int
    Name string
    Description string
    Location string
    DateTime time.Time
    UserID int 
}

// This is a variable to store the list of events
// slice literal
var events []Event = []Event{}

// This is a method that will be available to each instance of Event.
// It will save the the event to the database
func (e Event) Save() {
    // This will create a new underlying array,
    // that is made up of the events slice literal, plus the new event (e)
    events = append(events, e)
}

func GetAllEvents() []Event {
    return events
}

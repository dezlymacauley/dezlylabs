/*
    ABOUT: Channel vs Mutex

    Channel:
    A channel is used when you want to pass data or messages between threads,
    especially when the data is produced in one thread and consumed in
    another thread.
    
    This is great for tasks that can be neatly divided 
    and handled by different threads.

    Mutex (Short for mutal exclusion):
    This is for situations where mutliple threads need to access 
    and potentially modify shared data.

*/

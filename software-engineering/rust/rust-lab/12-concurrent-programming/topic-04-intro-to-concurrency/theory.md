What is concurrency?

How is is different but related to concepts like parallelism 
and asynchronus programming.

Thread management.

Thread safety.

1. Concurrency - This is about managing multiple tasks at once.

Concurrency involves task switching.

The CPU rapidly switches between different tasks, 
and this gives the illusion that each task 
is being completed simulataneously.

One a single core CPU this is achieved through time slicing.

On a multiple core CPU, task switching is performed across mutliple cores.

Rust's concurrency model ensures that the data is owned by one thread at
a time. This prevents data races.

2. Parallelism
This is when mutliple tasks are completed at the same time. The progress of 
one task does not depend on the other because this is not task switching.

True parallelism is not possible on a single core CPU.

This is because you need multiple cores to execute different tasks 
simulataneously.

3. Asynchronus Programming

This is often used with API calls.

This is when some task runs separately without blocking the main thread.
Then when that task is completed it notifies the main thread.


4. Threads
The code of a running program operates within a process.
Theses processes can run concurrently.

Inside a program, you can have independent parts that execute 
simulataneously. These parts are managed by a mechanism called a threads.


5. Channels are for thread to thread communication.

6. Shared state (Mutex and atomic types)

Mutex stands for "mutual exclusion lock", it is a way to ensure that only
one thread can access a piece of data at a time.

In Rust mutexes are smart, so when a mutex goes out of scope, 
it automatically releases its lock, which prevents common issues 
like deadlocks.

7. Atomic types - Used when you want to perform simple operations on shared
data without locking.

8. Race conditions - When two parts of your program try to interact with the
data in a way that leads to unpredictable results.

9. deadlocks - When two threads end up waiting for each other indefinately. 

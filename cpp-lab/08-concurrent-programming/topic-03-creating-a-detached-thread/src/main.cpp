#include <cstdio>

// Thread is a class that accepts a callable.
// A callable is any kind of callback such as a function pointer,
// a function object, or even a lambda function.

// This callable is the executed in a separate thread.

// When you create an instance of the thread class, 
// the constructor function does not wait for the thread to start.
// It immeadiately returns.
#include <cstdlib>
#include <thread>
#include <chrono>

using std::this_thread::sleep_for;
using std::chrono::seconds;

using std::thread;

void DownloadFile() {
    printf("Task B started. Downloading file...\n");
    
    // I will simulate a delay of 5 seconds.
    sleep_for(seconds(5));
   
    printf("Task B compeleted. File downloaded.\n");
    printf("\n");
}

int main() {
    printf("Task A started.\n");
    printf("Task A completed.\n");
    printf("\n");

    // This will create a new thread called `fileDownloadThread`,
    // and the `DownloadFile` function will be executed in that thread.
    thread fileDownloadThread(DownloadFile);
   
    // Task C can start and even complete, 
    // while task B is downloading the file in the background.
    printf("Task C started.\n");
    printf("Task C completed.\n");


    // NOTE: This is how you detach a thread from ???
    fileDownloadThread.detach();

    // Now the program will exit without any errors and without waiting
    // for the fileDownloadThread to complete.

    // So .detach() is for fire and forget tasks where the completion of
    // those tasks is not critical.
    
    // NOTE: .detach() is rare in production code. So don't worry too much
    // about it.
}

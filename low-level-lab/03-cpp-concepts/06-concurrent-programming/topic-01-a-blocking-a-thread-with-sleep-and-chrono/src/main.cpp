#include <cstdio>
#include <thread>
#include <chrono>

using std::this_thread::sleep_for;
using std::chrono::seconds;

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

    // This is a blocking function.
    // It will block the execution of code in the main thread until
    // the `DownloadFile` function completes.
    DownloadFile();
   
    // Task C will not start
    printf("Task C started.\n");
    printf("Task C completed.\n");
}

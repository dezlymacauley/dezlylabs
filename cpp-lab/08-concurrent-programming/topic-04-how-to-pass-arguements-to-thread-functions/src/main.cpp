#include <cstdio>
// NOTE: Not done
#include <string>
#include <thread>
#include <chrono>

using std::this_thread::sleep_for;
using std::chrono::seconds;

using std::thread;
using std::string;

void DownloadFile(string fileName) {
    // printf expects a 
    printf("Task B started. Downloading file %s...\n", fileName.c_str());
    
    // I will simulate a delay of 5 seconds.
    sleep_for(seconds(5));
   
    printf("Task B compeleted. File downloaded.\n");
    printf("\n");
}

int main() {
    printf("Task A started.\n");
    printf("Task A completed.\n");
    printf("\n");

    string fileName{"favourite_song.mp4"};
    
    // NOTE: This is how to pass a variable to a thread.

    // So the `DownloadFile` function will have access to the variable
    // `fileName`.
    thread fileDownloadThread(DownloadFile, fileName);
   
    // Task C can start and even complete, 
    // while task B is downloading the file in the background.
    printf("Task C started.\n");
    printf("Task C completed.\n");


    // NOTE: This is how you detach a thread from ???
    fileDownloadThread.join();

}

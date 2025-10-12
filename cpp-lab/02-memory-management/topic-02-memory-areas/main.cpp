#include <stdio.h>

#include <cstdlib>

int main() {
    // This is a pointer of the type
    int *p = (int *)malloc(sizeof(int));

    if (*p == nullptr) {
        // check if allocation succeeded
        return 1;
    }

    // *p = 5;
    printf("%d\n", *p);

    // free(p)
}

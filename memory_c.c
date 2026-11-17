#include <stdlib.h>

int main(void) {
    int *first = malloc(sizeof(int));
    if (first == NULL) return 1;

    *first = 10;
    int *second = first; // Both point to the same allocation.

    free(first);        // Frees the allocation.

    // *second would now access freed memory!
    // free(second) would be a double free.
}

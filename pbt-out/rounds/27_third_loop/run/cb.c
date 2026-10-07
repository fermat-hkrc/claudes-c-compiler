#include <stdio.h>
extern int common_int;          // merges with tentative
extern int defined_int;
extern int arr[];
extern int initialized[3];
int main(void) {
    common_int = 7;
    printf("%d %d %d %d %zu\n",
        common_int, defined_int, arr[50], initialized[2], sizeof(arr));
    return 0;
}

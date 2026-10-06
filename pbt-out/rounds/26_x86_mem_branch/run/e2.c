#include <stdio.h>
#include <string.h>
struct Triple { double x, y, z; };
struct Triple triple_arr[10];
void init_all(void) {
    struct Triple t = { 1.5, 2.5, 3.5 };
    for (int i = 0; i < 10; i++) {
        triple_arr[i] = t;  // struct copy in loop
        t.x += 1.0; t.y += 2.0; t.z += 3.0;
    }
}
int main(void) {
    init_all();
    double s = 0;
    for (int i = 0; i < 10; i++) {
        s += triple_arr[i].x + triple_arr[i].y * triple_arr[i].z;
    }
    printf("%.2f\n", s);
    return 0;
}

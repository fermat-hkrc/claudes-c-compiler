#include <stdio.h>
void copy_norestrict(int *a, int *b, int n) {
    for (int i = 0; i < n; i++) a[i] = b[i];
}
void copy_restrict(int *restrict a, const int *restrict b, int n) {
    for (int i = 0; i < n; i++) a[i] = b[i];
}
int main(void) {
    int src[10] = { 1,2,3,4,5,6,7,8,9,10 };
    int dst1[10] = { 0 }, dst2[10] = { 0 };
    copy_norestrict(dst1, src, 10);
    copy_restrict(dst2, src, 10);
    int same = 1;
    for (int i = 0; i < 10; i++) if (dst1[i] != dst2[i]) same = 0;
    printf("%d %d %d\n", same, dst1[5], dst2[5]);
    return 0;
}

static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[2] = cells[7] + 3;
    arr[2 % 6] = 658; cells[0] += arr[2 % 6];
    static int f1(int *p, int *q) { *p += 17; return *q * 2; }
    sum += f1(&cells[2], &cells[1]);
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 482u; }
    cells[4] = cells[0] + 778;
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    static int f2(int *p, int *q) { *p += 45; return *q * 2; }
    sum += f2(&cells[7], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
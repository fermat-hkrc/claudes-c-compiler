static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 117u; }
    cells[5] = cells[5] + 86;
    cells[6] = cells[3] + 835;
    static int f1(int *p, int *q) { *p += 695; return *q * 2; }
    sum += f1(&cells[3], &cells[0]);
    static int f2(int *p, int *q) { *p += 539; return *q * 2; }
    sum += f2(&cells[6], &cells[5]);
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 646u; }
    arr[6 % 6] = 249; cells[6] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    static int f3(int *p, int *q) { *p += 88; return *q * 2; }
    sum += f3(&cells[5], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
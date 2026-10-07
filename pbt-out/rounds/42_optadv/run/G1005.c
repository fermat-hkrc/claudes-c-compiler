static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 471; return *q * 2; }
    sum += f1(&cells[1], &cells[7]);
    static int f2(int *p, int *q) { *p += 473; return *q * 2; }
    sum += f2(&cells[6], &cells[7]);
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 457u; }
    static int f3(int *p, int *q) { *p += 520; return *q * 2; }
    sum += f3(&cells[5], &cells[0]);
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 398u; }
    static int f4(int *p, int *q) { *p += 656; return *q * 2; }
    sum += f4(&cells[0], &cells[1]);
    static int f5(int *p, int *q) { *p += 328; return *q * 2; }
    sum += f5(&cells[0], &cells[7]);
    arr[5 % 6] = 817; cells[6] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
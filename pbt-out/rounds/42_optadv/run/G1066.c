static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    arr[2 % 6] = 425; cells[6] += arr[2 % 6];
    static int f1(int *p, int *q) { *p += 89; return *q * 2; }
    sum += f1(&cells[3], &cells[0]);
    cells[7] = cells[1] + 636;
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 741u; }
    static int f2(int *p, int *q) { *p += 893; return *q * 2; }
    sum += f2(&cells[6], &cells[6]);
    static int f3(int *p, int *q) { *p += 577; return *q * 2; }
    sum += f3(&cells[4], &cells[2]);
    arr[4 % 6] = 926; cells[5] += arr[4 % 6];
    arr[1 % 6] = 265; cells[2] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    static int f4(int *p, int *q) { *p += 830; return *q * 2; }
    sum += f4(&cells[1], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
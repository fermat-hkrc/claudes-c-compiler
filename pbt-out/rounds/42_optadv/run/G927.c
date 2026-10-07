static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 398u; }
    cells[0] = cells[1] + 805;
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    arr[3 % 6] = 619; cells[4] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 457; return *q * 2; }
    sum += f1(&cells[3], &cells[4]);
    cells[4] = cells[4] + 153;
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    static int f2(int *p, int *q) { *p += 227; return *q * 2; }
    sum += f2(&cells[3], &cells[2]);
    static int f3(int *p, int *q) { *p += 913; return *q * 2; }
    sum += f3(&cells[7], &cells[4]);
    cells[3] = cells[0] + 103;
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    arr[1 % 6] = 803; cells[5] += arr[1 % 6];
    cells[2] = cells[1] + 426;
    static int f4(int *p, int *q) { *p += 908; return *q * 2; }
    sum += f4(&cells[4], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
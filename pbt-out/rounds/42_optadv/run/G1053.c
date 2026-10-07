static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 133u; }
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    cells[3] = cells[2] + 682;
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    static int f1(int *p, int *q) { *p += 592; return *q * 2; }
    sum += f1(&cells[4], &cells[1]);
    static int f2(int *p, int *q) { *p += 794; return *q * 2; }
    sum += f2(&cells[5], &cells[4]);
    cells[1] = cells[0] + 447;
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 165u; }
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 63u; }
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 658u; }
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    static int f3(int *p, int *q) { *p += 830; return *q * 2; }
    sum += f3(&cells[0], &cells[5]);
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 473u; }
    arr[6 % 6] = 332; cells[3] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[2] = cells[7] + 137;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 713u; }
    static int f4(int *p, int *q) { *p += 839; return *q * 2; }
    sum += f4(&cells[2], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
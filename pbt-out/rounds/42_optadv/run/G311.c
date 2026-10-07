static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 198; cells[6] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    static int f1(int *p, int *q) { *p += 572; return *q * 2; }
    sum += f1(&cells[6], &cells[2]);
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 570u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f2(int *p, int *q) { *p += 121; return *q * 2; }
    sum += f2(&cells[2], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f3(int *p, int *q) { *p += 970; return *q * 2; }
    sum += f3(&cells[0], &cells[3]);
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    static int f4(int *p, int *q) { *p += 907; return *q * 2; }
    sum += f4(&cells[5], &cells[1]);
    cells[0] = cells[6] + 118;
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 903u; }
    cells[6] = cells[7] + 581;
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    cells[1] = cells[5] + 830;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    static int f5(int *p, int *q) { *p += 175; return *q * 2; }
    sum += f5(&cells[6], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
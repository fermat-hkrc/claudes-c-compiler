static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[4] = cells[1] + 466;
    cells[3] = cells[6] + 639;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f1(int *p, int *q) { *p += 780; return *q * 2; }
    sum += f1(&cells[6], &cells[6]);
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    cells[1] = cells[3] + 765;
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 221u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    static int f2(int *p, int *q) { *p += 144; return *q * 2; }
    sum += f2(&cells[5], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 792u; }
    static int f3(int *p, int *q) { *p += 820; return *q * 2; }
    sum += f3(&cells[1], &cells[4]);
    arr[0 % 6] = 75; cells[5] += arr[0 % 6];
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 545u; }
    cells[2] = cells[3] + 969;
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    static int f4(int *p, int *q) { *p += 756; return *q * 2; }
    sum += f4(&cells[1], &cells[6]);
    static int f5(int *p, int *q) { *p += 537; return *q * 2; }
    sum += f5(&cells[6], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
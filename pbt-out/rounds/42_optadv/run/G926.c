static int f1(int *p, int *q) { *p += 369; return *q * 2; }
static int f2(int *p, int *q) { *p += 260; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[3], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f2(&cells[6], &cells[5]);
    cells[7] = cells[1] + 156;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    cells[1] = cells[5] + 589;
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 481u; }
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 384u; }
    cells[5] = cells[6] + 746;
    arr[2 % 6] = 327; cells[6] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[6] = cells[6] + 429;
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 821u; }
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
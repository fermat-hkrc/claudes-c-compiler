static int f1(int *p, int *q) { *p += 866; return *q * 2; }
static int f2(int *p, int *q) { *p += 16; return *q * 2; }
static int f3(int *p, int *q) { *p += 885; return *q * 2; }
static int f4(int *p, int *q) { *p += 494; return *q * 2; }
static int f5(int *p, int *q) { *p += 104; return *q * 2; }
static int f6(int *p, int *q) { *p += 798; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[2] + 944;
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[1] = cells[4] + 305;
    sum += f1(&cells[3], &cells[5]);
    sum += f2(&cells[7], &cells[3]);
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 891u; }
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 429u; }
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    sum += f3(&cells[7], &cells[1]);
    arr[5 % 6] = 583; cells[6] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    sum += f4(&cells[6], &cells[6]);
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    sum += f5(&cells[4], &cells[0]);
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 398u; }
    sum += f6(&cells[7], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
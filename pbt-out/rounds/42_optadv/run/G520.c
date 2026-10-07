static int f1(int *p, int *q) { *p += 680; return *q * 2; }
static int f2(int *p, int *q) { *p += 395; return *q * 2; }
static int f3(int *p, int *q) { *p += 260; return *q * 2; }
static int f4(int *p, int *q) { *p += 951; return *q * 2; }
static int f5(int *p, int *q) { *p += 320; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 119u; }
    cells[1] = cells[0] + 705;
    sum += f1(&cells[1], &cells[0]);
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    sum += f2(&cells[5], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f3(&cells[0], &cells[1]);
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 294u; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += f4(&cells[5], &cells[3]);
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    sum += f5(&cells[3], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[0 % 6] = 552; cells[5] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
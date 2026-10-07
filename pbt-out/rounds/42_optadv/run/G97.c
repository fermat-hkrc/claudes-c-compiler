static int f1(int *p, int *q) { *p += 713; return *q * 2; }
static int f2(int *p, int *q) { *p += 672; return *q * 2; }
static int f3(int *p, int *q) { *p += 936; return *q * 2; }
static int f4(int *p, int *q) { *p += 589; return *q * 2; }
static int f5(int *p, int *q) { *p += 949; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[6] = cells[0] + 339;
    sum += f1(&cells[3], &cells[6]);
    cells[7] = cells[6] + 14;
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 231u; }
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += f2(&cells[5], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f3(&cells[0], &cells[3]);
    cells[5] = cells[1] + 831;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 243u; }
    sum += f4(&cells[4], &cells[7]);
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    sum += f5(&cells[0], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
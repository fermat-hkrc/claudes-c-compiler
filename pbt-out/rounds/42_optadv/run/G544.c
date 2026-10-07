static int f1(int *p, int *q) { *p += 365; return *q * 2; }
static int f2(int *p, int *q) { *p += 99; return *q * 2; }
static int f3(int *p, int *q) { *p += 890; return *q * 2; }
static int f4(int *p, int *q) { *p += 715; return *q * 2; }
static int f5(int *p, int *q) { *p += 343; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 827u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    cells[5] = cells[4] + 983;
    cells[1] = cells[2] + 569;
    sum += f1(&cells[5], &cells[3]);
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    cells[4] = cells[3] + 582;
    arr[0 % 6] = 481; cells[5] += arr[0 % 6];
    sum += f2(&cells[2], &cells[2]);
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    cells[7] = cells[2] + 865;
    sum += f3(&cells[1], &cells[3]);
    sum += f4(&cells[3], &cells[5]);
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    sum += f5(&cells[3], &cells[7]);
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[3] = cells[5] + 13;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 653; return *q * 2; }
static int f2(int *p, int *q) { *p += 863; return *q * 2; }
static int f3(int *p, int *q) { *p += 934; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 327; cells[1] += arr[1 % 6];
    sum += f1(&cells[4], &cells[1]);
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f2(&cells[3], &cells[4]);
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 389u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 10u; }
    sum += f3(&cells[4], &cells[0]);
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 635u; }
    arr[2 % 6] = 578; cells[4] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
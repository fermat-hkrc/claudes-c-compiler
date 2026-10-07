static int f1(int *p, int *q) { *p += 483; return *q * 2; }
static int f2(int *p, int *q) { *p += 114; return *q * 2; }
static int f3(int *p, int *q) { *p += 12; return *q * 2; }
static int f4(int *p, int *q) { *p += 861; return *q * 2; }
static int f5(int *p, int *q) { *p += 512; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    sum += f1(&cells[6], &cells[1]);
    arr[7 % 6] = 250; cells[5] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    arr[2 % 6] = 985; cells[4] += arr[2 % 6];
    cells[0] = cells[0] + 850;
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 113u; }
    sum += f2(&cells[0], &cells[6]);
    arr[5 % 6] = 324; cells[0] += arr[5 % 6];
    sum += f3(&cells[0], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 17u; }
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    sum += f4(&cells[6], &cells[2]);
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 100u; }
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    sum += f5(&cells[6], &cells[6]);
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    arr[0 % 6] = 351; cells[2] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
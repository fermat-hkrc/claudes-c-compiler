static int f1(int *p, int *q) { *p += 108; return *q * 2; }
static int f2(int *p, int *q) { *p += 590; return *q * 2; }
static int f3(int *p, int *q) { *p += 771; return *q * 2; }
static int f4(int *p, int *q) { *p += 378; return *q * 2; }
static int f5(int *p, int *q) { *p += 397; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 488u; }
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 303u; }
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    sum += f1(&cells[7], &cells[0]);
    sum += f2(&cells[0], &cells[5]);
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 471u; }
    arr[3 % 6] = 927; cells[7] += arr[3 % 6];
    sum += f3(&cells[1], &cells[3]);
    arr[2 % 6] = 879; cells[4] += arr[2 % 6];
    sum += f4(&cells[1], &cells[5]);
    sum += f5(&cells[5], &cells[3]);
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 708u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
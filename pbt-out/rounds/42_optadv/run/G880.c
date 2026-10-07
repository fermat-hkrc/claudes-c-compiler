static int f1(int *p, int *q) { *p += 353; return *q * 2; }
static int f2(int *p, int *q) { *p += 241; return *q * 2; }
static int f3(int *p, int *q) { *p += 774; return *q * 2; }
static int f4(int *p, int *q) { *p += 349; return *q * 2; }
static int f5(int *p, int *q) { *p += 687; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    sum += f1(&cells[3], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f2(&cells[4], &cells[0]);
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    sum += f3(&cells[1], &cells[7]);
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 438u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[0] = cells[2] + 119;
    cells[7] = cells[4] + 331;
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 837u; }
    arr[6 % 6] = 68; cells[6] += arr[6 % 6];
    sum += f4(&cells[4], &cells[4]);
    sum += f5(&cells[3], &cells[6]);
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 4u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
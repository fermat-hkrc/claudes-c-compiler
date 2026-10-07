static int f1(int *p, int *q) { *p += 609; return *q * 2; }
static int f2(int *p, int *q) { *p += 782; return *q * 2; }
static int f3(int *p, int *q) { *p += 662; return *q * 2; }
static int f4(int *p, int *q) { *p += 571; return *q * 2; }
static int f5(int *p, int *q) { *p += 488; return *q * 2; }
static int f6(int *p, int *q) { *p += 890; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f1(&cells[5], &cells[3]);
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 196u; }
    cells[4] = cells[3] + 544;
    sum += f2(&cells[4], &cells[7]);
    sum += f3(&cells[6], &cells[0]);
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 475u; }
    cells[3] = cells[6] + 475;
    sum += f4(&cells[5], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    arr[5 % 6] = 281; cells[0] += arr[5 % 6];
    arr[3 % 6] = 40; cells[5] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f5(&cells[3], &cells[6]);
    cells[1] = cells[0] + 698;
    sum += f6(&cells[5], &cells[3]);
    cells[5] = cells[6] + 804;
    cells[5] = cells[3] + 105;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
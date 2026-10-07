static int f1(int *p, int *q) { *p += 843; return *q * 2; }
static int f2(int *p, int *q) { *p += 521; return *q * 2; }
static int f3(int *p, int *q) { *p += 139; return *q * 2; }
static int f4(int *p, int *q) { *p += 126; return *q * 2; }
static int f5(int *p, int *q) { *p += 132; return *q * 2; }
static int f6(int *p, int *q) { *p += 421; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    sum += f1(&cells[7], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 533u; }
    arr[6 % 6] = 185; cells[3] += arr[6 % 6];
    sum += f2(&cells[7], &cells[6]);
    sum += f3(&cells[3], &cells[6]);
    sum += f4(&cells[5], &cells[7]);
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    arr[4 % 6] = 926; cells[5] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f5(&cells[4], &cells[5]);
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    cells[6] = cells[5] + 187;
    cells[3] = cells[4] + 777;
    cells[5] = cells[2] + 435;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f6(&cells[3], &cells[5]);
    arr[5 % 6] = 393; cells[4] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
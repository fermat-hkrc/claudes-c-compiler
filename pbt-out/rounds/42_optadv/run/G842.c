static int f1(int *p, int *q) { *p += 635; return *q * 2; }
static int f2(int *p, int *q) { *p += 387; return *q * 2; }
static int f3(int *p, int *q) { *p += 983; return *q * 2; }
static int f4(int *p, int *q) { *p += 559; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[4], &cells[1]);
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 807u; }
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 867u; }
    sum += f2(&cells[1], &cells[6]);
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[7] = cells[4] + 586;
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    cells[4] = cells[6] + 529;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 970u; }
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    sum += f3(&cells[6], &cells[6]);
    cells[5] = cells[6] + 289;
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    arr[5 % 6] = 976; cells[7] += arr[5 % 6];
    arr[3 % 6] = 478; cells[5] += arr[3 % 6];
    cells[7] = cells[5] + 322;
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 452u; }
    sum += f4(&cells[4], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
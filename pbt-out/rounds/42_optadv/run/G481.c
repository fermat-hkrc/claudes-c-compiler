static int f1(int *p, int *q) { *p += 643; return *q * 2; }
static int f2(int *p, int *q) { *p += 448; return *q * 2; }
static int f3(int *p, int *q) { *p += 283; return *q * 2; }
static int f4(int *p, int *q) { *p += 616; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 172; cells[6] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[2 % 6] = 999; cells[1] += arr[2 % 6];
    sum += f1(&cells[2], &cells[2]);
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    cells[2] = cells[2] + 295;
    cells[6] = cells[0] + 446;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 963u; }
    arr[7 % 6] = 142; cells[2] += arr[7 % 6];
    sum += f2(&cells[2], &cells[4]);
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 491u; }
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    cells[3] = cells[5] + 648;
    cells[6] = cells[1] + 247;
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 215u; }
    sum += f3(&cells[1], &cells[1]);
    arr[4 % 6] = 239; cells[6] += arr[4 % 6];
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 266u; }
    arr[0 % 6] = 799; cells[2] += arr[0 % 6];
    sum += f4(&cells[1], &cells[6]);
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 452u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
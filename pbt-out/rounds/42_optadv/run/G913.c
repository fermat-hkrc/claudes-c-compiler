static int f1(int *p, int *q) { *p += 797; return *q * 2; }
static int f2(int *p, int *q) { *p += 453; return *q * 2; }
static int f3(int *p, int *q) { *p += 593; return *q * 2; }
static int f4(int *p, int *q) { *p += 764; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[7] + 121;
    sum += f1(&cells[4], &cells[7]);
    sum += f2(&cells[3], &cells[7]);
    cells[2] = cells[4] + 201;
    arr[5 % 6] = 196; cells[4] += arr[5 % 6];
    cells[7] = cells[5] + 536;
    sum += f3(&cells[7], &cells[0]);
    arr[4 % 6] = 267; cells[6] += arr[4 % 6];
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 282u; }
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f4(&cells[6], &cells[7]);
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[2 % 6] = 813; cells[5] += arr[2 % 6];
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 127u; }
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    arr[1 % 6] = 382; cells[7] += arr[1 % 6];
    arr[1 % 6] = 438; cells[5] += arr[1 % 6];
    arr[4 % 6] = 197; cells[4] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 758u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
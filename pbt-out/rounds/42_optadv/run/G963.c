static int f1(int *p, int *q) { *p += 662; return *q * 2; }
static int f2(int *p, int *q) { *p += 199; return *q * 2; }
static int f3(int *p, int *q) { *p += 443; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 997u; }
    sum += f1(&cells[1], &cells[7]);
    arr[2 % 6] = 430; cells[6] += arr[2 % 6];
    sum += f2(&cells[2], &cells[6]);
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 194u; }
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    arr[6 % 6] = 667; cells[3] += arr[6 % 6];
    sum += f3(&cells[2], &cells[5]);
    arr[6 % 6] = 444; cells[2] += arr[6 % 6];
    arr[5 % 6] = 123; cells[4] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[4 % 6] = 658; cells[3] += arr[4 % 6];
    cells[3] = cells[6] + 15;
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 172u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[5] = cells[2] + 986;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
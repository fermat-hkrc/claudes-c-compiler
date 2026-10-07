static int f1(int *p, int *q) { *p += 29; return *q * 2; }
static int f2(int *p, int *q) { *p += 595; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    cells[2] = cells[4] + 987;
    arr[7 % 6] = 110; cells[1] += arr[7 % 6];
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 247u; }
    sum += f1(&cells[2], &cells[0]);
    sum += f2(&cells[3], &cells[2]);
    arr[7 % 6] = 984; cells[2] += arr[7 % 6];
    cells[1] = cells[7] + 542;
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    arr[0 % 6] = 938; cells[6] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
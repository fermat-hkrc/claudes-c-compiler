static int f1(int *p, int *q) { *p += 306; return *q * 2; }
static int f2(int *p, int *q) { *p += 853; return *q * 2; }
static int f3(int *p, int *q) { *p += 548; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[3] + 221;
    sum += f1(&cells[5], &cells[6]);
    cells[7] = cells[7] + 667;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    arr[2 % 6] = 340; cells[6] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    sum += f2(&cells[6], &cells[5]);
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 42u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 82u; }
    arr[3 % 6] = 342; cells[5] += arr[3 % 6];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    sum += f3(&cells[2], &cells[6]);
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    cells[7] = cells[1] + 248;
    cells[5] = cells[3] + 906;
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 661u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
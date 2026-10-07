static int f1(int *p, int *q) { *p += 425; return *q * 2; }
static int f2(int *p, int *q) { *p += 546; return *q * 2; }
static int f3(int *p, int *q) { *p += 841; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 75u; }
    arr[2 % 6] = 526; cells[6] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[3] = cells[7] + 643;
    cells[2] = cells[7] + 26;
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    sum += f1(&cells[6], &cells[6]);
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 413u; }
    cells[5] = cells[6] + 718;
    sum += f2(&cells[4], &cells[7]);
    sum += f3(&cells[2], &cells[5]);
    arr[1 % 6] = 24; cells[4] += arr[1 % 6];
    arr[5 % 6] = 44; cells[4] += arr[5 % 6];
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 151u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    arr[3 % 6] = 697; cells[2] += arr[3 % 6];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 163u; }
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 915u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
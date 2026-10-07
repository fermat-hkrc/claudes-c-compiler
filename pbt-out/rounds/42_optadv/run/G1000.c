static int f1(int *p, int *q) { *p += 532; return *q * 2; }
static int f2(int *p, int *q) { *p += 844; return *q * 2; }
static int f3(int *p, int *q) { *p += 902; return *q * 2; }
static int f4(int *p, int *q) { *p += 104; return *q * 2; }
static int f5(int *p, int *q) { *p += 464; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    cells[2] = cells[1] + 351;
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 164u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 858u; }
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    arr[5 % 6] = 892; cells[4] += arr[5 % 6];
    sum += f1(&cells[3], &cells[6]);
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 745u; }
    cells[1] = cells[6] + 789;
    arr[7 % 6] = 242; cells[7] += arr[7 % 6];
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    sum += f2(&cells[0], &cells[6]);
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 563u; }
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    sum += f3(&cells[1], &cells[6]);
    cells[3] = cells[2] + 176;
    sum += f4(&cells[4], &cells[3]);
    sum += f5(&cells[5], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
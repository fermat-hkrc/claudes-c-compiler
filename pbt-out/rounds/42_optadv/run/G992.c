static int f1(int *p, int *q) { *p += 238; return *q * 2; }
static int f2(int *p, int *q) { *p += 789; return *q * 2; }
static int f3(int *p, int *q) { *p += 959; return *q * 2; }
static int f4(int *p, int *q) { *p += 694; return *q * 2; }
static int f5(int *p, int *q) { *p += 312; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 81; cells[3] += arr[7 % 6];
    sum += f1(&cells[4], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    arr[2 % 6] = 384; cells[1] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    sum += f2(&cells[0], &cells[1]);
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    cells[6] = cells[5] + 395;
    sum += f3(&cells[5], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    arr[3 % 6] = 537; cells[5] += arr[3 % 6];
    sum += f4(&cells[0], &cells[3]);
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 731u; }
    arr[2 % 6] = 262; cells[2] += arr[2 % 6];
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    arr[4 % 6] = 295; cells[5] += arr[4 % 6];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 370u; }
    sum += f5(&cells[0], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
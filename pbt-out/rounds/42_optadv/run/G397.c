static int f1(int *p, int *q) { *p += 953; return *q * 2; }
static int f2(int *p, int *q) { *p += 991; return *q * 2; }
static int f3(int *p, int *q) { *p += 229; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 225; cells[7] += arr[6 % 6];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 314u; }
    arr[1 % 6] = 675; cells[0] += arr[1 % 6];
    cells[6] = cells[2] + 524;
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    arr[6 % 6] = 353; cells[7] += arr[6 % 6];
    sum += f1(&cells[6], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 216u; }
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 762u; }
    arr[6 % 6] = 660; cells[4] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    sum += f2(&cells[6], &cells[1]);
    arr[5 % 6] = 490; cells[5] += arr[5 % 6];
    cells[5] = cells[4] + 973;
    arr[3 % 6] = 850; cells[7] += arr[3 % 6];
    arr[7 % 6] = 174; cells[1] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    sum += f3(&cells[5], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
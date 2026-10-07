static int f1(int *p, int *q) { *p += 677; return *q * 2; }
static int f2(int *p, int *q) { *p += 46; return *q * 2; }
static int f3(int *p, int *q) { *p += 902; return *q * 2; }
static int f4(int *p, int *q) { *p += 791; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 736; cells[0] += arr[7 % 6];
    arr[4 % 6] = 351; cells[5] += arr[4 % 6];
    sum += f1(&cells[4], &cells[3]);
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    arr[7 % 6] = 235; cells[5] += arr[7 % 6];
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 905u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    sum += f2(&cells[5], &cells[1]);
    arr[5 % 6] = 90; cells[4] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 421u; }
    sum += f3(&cells[4], &cells[0]);
    sum += f4(&cells[6], &cells[4]);
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    cells[7] = cells[3] + 785;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    cells[1] = cells[2] + 424;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
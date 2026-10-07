static int f1(int *p, int *q) { *p += 821; return *q * 2; }
static int f2(int *p, int *q) { *p += 748; return *q * 2; }
static int f3(int *p, int *q) { *p += 501; return *q * 2; }
static int f4(int *p, int *q) { *p += 177; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[2] = cells[2] + 9;
    sum += f1(&cells[5], &cells[4]);
    sum += f2(&cells[2], &cells[0]);
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 804u; }
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 450u; }
    arr[3 % 6] = 950; cells[0] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    sum += f3(&cells[2], &cells[4]);
    cells[0] = cells[5] + 91;
    arr[0 % 6] = 394; cells[4] += arr[0 % 6];
    sum += f4(&cells[0], &cells[0]);
    arr[0 % 6] = 2; cells[6] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
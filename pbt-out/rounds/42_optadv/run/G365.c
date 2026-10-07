static int f1(int *p, int *q) { *p += 95; return *q * 2; }
static int f2(int *p, int *q) { *p += 570; return *q * 2; }
static int f3(int *p, int *q) { *p += 408; return *q * 2; }
static int f4(int *p, int *q) { *p += 275; return *q * 2; }
static int f5(int *p, int *q) { *p += 331; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 748; cells[3] += arr[3 % 6];
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 928u; }
    arr[7 % 6] = 89; cells[3] += arr[7 % 6];
    sum += f1(&cells[5], &cells[6]);
    arr[4 % 6] = 260; cells[1] += arr[4 % 6];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += f2(&cells[1], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    sum += f3(&cells[6], &cells[6]);
    arr[3 % 6] = 154; cells[0] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    arr[5 % 6] = 106; cells[3] += arr[5 % 6];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    sum += f4(&cells[1], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[1] = cells[7] + 803;
    sum += f5(&cells[1], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
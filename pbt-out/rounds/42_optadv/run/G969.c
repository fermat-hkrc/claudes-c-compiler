static int f1(int *p, int *q) { *p += 53; return *q * 2; }
static int f2(int *p, int *q) { *p += 435; return *q * 2; }
static int f3(int *p, int *q) { *p += 795; return *q * 2; }
static int f4(int *p, int *q) { *p += 898; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 972u; }
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    arr[6 % 6] = 720; cells[5] += arr[6 % 6];
    cells[4] = cells[0] + 570;
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    sum += f1(&cells[1], &cells[6]);
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    sum += f2(&cells[2], &cells[3]);
    sum += f3(&cells[1], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    sum += f4(&cells[4], &cells[5]);
    arr[4 % 6] = 632; cells[6] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
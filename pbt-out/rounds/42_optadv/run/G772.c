static int f1(int *p, int *q) { *p += 350; return *q * 2; }
static int f2(int *p, int *q) { *p += 443; return *q * 2; }
static int f3(int *p, int *q) { *p += 250; return *q * 2; }
static int f4(int *p, int *q) { *p += 462; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    sum += f1(&cells[3], &cells[6]);
    arr[5 % 6] = 904; cells[5] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 230u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 665u; }
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    sum += f2(&cells[7], &cells[2]);
    sum += f3(&cells[3], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[7 % 6] = 369; cells[0] += arr[7 % 6];
    cells[1] = cells[2] + 567;
    cells[5] = cells[2] + 475;
    sum += f4(&cells[4], &cells[0]);
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
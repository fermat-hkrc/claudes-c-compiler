static int f1(int *p, int *q) { *p += 144; return *q * 2; }
static int f2(int *p, int *q) { *p += 428; return *q * 2; }
static int f3(int *p, int *q) { *p += 457; return *q * 2; }
static int f4(int *p, int *q) { *p += 583; return *q * 2; }
static int f5(int *p, int *q) { *p += 337; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[5] = cells[7] + 303;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f1(&cells[6], &cells[0]);
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    sum += f2(&cells[5], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[5] = cells[5] + 285;
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 869u; }
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 453u; }
    sum += f3(&cells[1], &cells[2]);
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    cells[7] = cells[2] + 836;
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    cells[0] = cells[5] + 86;
    sum += f4(&cells[7], &cells[6]);
    arr[6 % 6] = 895; cells[5] += arr[6 % 6];
    cells[4] = cells[3] + 342;
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    arr[4 % 6] = 410; cells[4] += arr[4 % 6];
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    sum += f5(&cells[1], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
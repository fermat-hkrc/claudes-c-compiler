static int f1(int *p, int *q) { *p += 397; return *q * 2; }
static int f2(int *p, int *q) { *p += 707; return *q * 2; }
static int f3(int *p, int *q) { *p += 981; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 67u; }
    sum += f1(&cells[1], &cells[1]);
    sum += f2(&cells[0], &cells[2]);
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 124u; }
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    sum += f3(&cells[5], &cells[1]);
    cells[3] = cells[3] + 3;
    arr[4 % 6] = 88; cells[4] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 812u; }
    cells[7] = cells[0] + 760;
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 405u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[7 % 6] = 615; cells[6] += arr[7 % 6];
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
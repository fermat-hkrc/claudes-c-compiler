static int f1(int *p, int *q) { *p += 0; return *q * 2; }
static int f2(int *p, int *q) { *p += 450; return *q * 2; }
static int f3(int *p, int *q) { *p += 176; return *q * 2; }
static int f4(int *p, int *q) { *p += 540; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[3], &cells[2]);
    sum += f2(&cells[5], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 744u; }
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    arr[7 % 6] = 881; cells[0] += arr[7 % 6];
    arr[5 % 6] = 494; cells[3] += arr[5 % 6];
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 56u; }
    cells[6] = cells[3] + 126;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 808u; }
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 439u; }
    sum += f3(&cells[1], &cells[6]);
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 645u; }
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    cells[4] = cells[2] + 411;
    sum += f4(&cells[3], &cells[1]);
    cells[3] = cells[0] + 914;
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    arr[2 % 6] = 786; cells[3] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
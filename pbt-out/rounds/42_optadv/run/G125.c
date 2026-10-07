static int f1(int *p, int *q) { *p += 777; return *q * 2; }
static int f2(int *p, int *q) { *p += 978; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 33u; }
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 800u; }
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    arr[6 % 6] = 176; cells[5] += arr[6 % 6];
    cells[0] = cells[4] + 52;
    cells[3] = cells[3] + 751;
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[1] = cells[2] + 54;
    arr[3 % 6] = 513; cells[3] += arr[3 % 6];
    sum += f1(&cells[6], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 775u; }
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 790u; }
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 76u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    sum += f2(&cells[6], &cells[0]);
    cells[0] = cells[3] + 98;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
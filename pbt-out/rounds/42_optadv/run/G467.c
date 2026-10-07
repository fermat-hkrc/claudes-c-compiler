static int f1(int *p, int *q) { *p += 405; return *q * 2; }
static int f2(int *p, int *q) { *p += 196; return *q * 2; }
static int f3(int *p, int *q) { *p += 181; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 517u; }
    arr[3 % 6] = 559; cells[3] += arr[3 % 6];
    arr[5 % 6] = 693; cells[3] += arr[5 % 6];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    arr[4 % 6] = 181; cells[1] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 294u; }
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    sum += f1(&cells[7], &cells[7]);
    arr[4 % 6] = 198; cells[4] += arr[4 % 6];
    cells[2] = cells[6] + 509;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f2(&cells[1], &cells[7]);
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 768u; }
    sum += f3(&cells[2], &cells[0]);
    cells[7] = cells[3] + 782;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 456; return *q * 2; }
static int f2(int *p, int *q) { *p += 117; return *q * 2; }
static int f3(int *p, int *q) { *p += 24; return *q * 2; }
static int f4(int *p, int *q) { *p += 20; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[6]);
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 856u; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    sum += f2(&cells[7], &cells[7]);
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    cells[6] = cells[4] + 118;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[4 % 6] = 84; cells[2] += arr[4 % 6];
    cells[7] = cells[4] + 158;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[5 % 6] = 304; cells[2] += arr[5 % 6];
    arr[4 % 6] = 403; cells[2] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[5] = cells[5] + 389;
    sum += f3(&cells[6], &cells[7]);
    sum += f4(&cells[4], &cells[0]);
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    arr[1 % 6] = 372; cells[7] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
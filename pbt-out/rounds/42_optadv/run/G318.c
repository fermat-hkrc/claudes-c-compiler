static int f1(int *p, int *q) { *p += 852; return *q * 2; }
static int f2(int *p, int *q) { *p += 667; return *q * 2; }
static int f3(int *p, int *q) { *p += 793; return *q * 2; }
static int f4(int *p, int *q) { *p += 313; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 76u; }
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    arr[4 % 6] = 918; cells[3] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    cells[4] = cells[0] + 11;
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 359u; }
    cells[7] = cells[0] + 248;
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    arr[3 % 6] = 404; cells[6] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f1(&cells[5], &cells[7]);
    sum += f2(&cells[3], &cells[5]);
    sum += f3(&cells[2], &cells[3]);
    arr[0 % 6] = 479; cells[1] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f4(&cells[2], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
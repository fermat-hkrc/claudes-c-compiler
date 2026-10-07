static int f1(int *p, int *q) { *p += 663; return *q * 2; }
static int f2(int *p, int *q) { *p += 985; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += f1(&cells[0], &cells[0]);
    arr[4 % 6] = 940; cells[7] += arr[4 % 6];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 359u; }
    arr[2 % 6] = 481; cells[2] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    cells[7] = cells[3] + 555;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 815u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[3 % 6] = 926; cells[7] += arr[3 % 6];
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 280u; }
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 561u; }
    arr[4 % 6] = 798; cells[4] += arr[4 % 6];
    sum += f2(&cells[1], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
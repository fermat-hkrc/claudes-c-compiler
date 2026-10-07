static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[0 % 6] = 319; cells[1] += arr[0 % 6];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    static int f1(int *p, int *q) { *p += 329; return *q * 2; }
    sum += f1(&cells[5], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[1] = cells[4] + 541;
    static int f2(int *p, int *q) { *p += 565; return *q * 2; }
    sum += f2(&cells[4], &cells[1]);
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    arr[2 % 6] = 351; cells[1] += arr[2 % 6];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 219u; }
    static int f3(int *p, int *q) { *p += 118; return *q * 2; }
    sum += f3(&cells[4], &cells[2]);
    cells[1] = cells[6] + 59;
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 150u; }
    static int f4(int *p, int *q) { *p += 120; return *q * 2; }
    sum += f4(&cells[5], &cells[4]);
    arr[4 % 6] = 565; cells[6] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 592u; }
    arr[1 % 6] = 869; cells[4] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    static int f1(int *p, int *q) { *p += 442; return *q * 2; }
    sum += f1(&cells[7], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    arr[1 % 6] = 507; cells[0] += arr[1 % 6];
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    static int f2(int *p, int *q) { *p += 644; return *q * 2; }
    sum += f2(&cells[0], &cells[0]);
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    cells[2] = cells[2] + 945;
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    arr[2 % 6] = 9; cells[6] += arr[2 % 6];
    arr[0 % 6] = 88; cells[5] += arr[0 % 6];
    static int f3(int *p, int *q) { *p += 535; return *q * 2; }
    sum += f3(&cells[7], &cells[1]);
    arr[7 % 6] = 112; cells[3] += arr[7 % 6];
    static int f4(int *p, int *q) { *p += 446; return *q * 2; }
    sum += f4(&cells[1], &cells[1]);
    arr[5 % 6] = 364; cells[0] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
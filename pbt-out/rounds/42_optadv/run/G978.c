static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[4] = cells[5] + 528;
    static int f1(int *p, int *q) { *p += 583; return *q * 2; }
    sum += f1(&cells[3], &cells[0]);
    arr[2 % 6] = 701; cells[1] += arr[2 % 6];
    static int f2(int *p, int *q) { *p += 71; return *q * 2; }
    sum += f2(&cells[7], &cells[0]);
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 632u; }
    static int f3(int *p, int *q) { *p += 711; return *q * 2; }
    sum += f3(&cells[1], &cells[4]);
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 906u; }
    static int f4(int *p, int *q) { *p += 752; return *q * 2; }
    sum += f4(&cells[3], &cells[5]);
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    cells[4] = cells[5] + 256;
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 259u; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 578u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[3 % 6] = 461; cells[1] += arr[3 % 6];
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    arr[2 % 6] = 466; cells[0] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    cells[1] = cells[3] + 506;
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[1 % 6] = 298; cells[1] += arr[1 % 6];
    arr[4 % 6] = 651; cells[4] += arr[4 % 6];
    static int f1(int *p, int *q) { *p += 249; return *q * 2; }
    sum += f1(&cells[2], &cells[3]);
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    cells[1] = cells[3] + 398;
    static int f2(int *p, int *q) { *p += 289; return *q * 2; }
    sum += f2(&cells[6], &cells[3]);
    arr[0 % 6] = 3; cells[4] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 192u; }
    cells[1] = cells[2] + 907;
    static int f3(int *p, int *q) { *p += 66; return *q * 2; }
    sum += f3(&cells[7], &cells[7]);
    arr[6 % 6] = 652; cells[4] += arr[6 % 6];
    cells[4] = cells[3] + 469;
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 682u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[3] = cells[0] + 23;
    cells[6] = cells[6] + 958;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
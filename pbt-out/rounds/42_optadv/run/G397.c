static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 335u; }
    cells[1] = cells[5] + 234;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[3 % 6] = 234; cells[0] += arr[3 % 6];
    cells[5] = cells[2] + 282;
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 245u; }
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    arr[1 % 6] = 335; cells[4] += arr[1 % 6];
    arr[3 % 6] = 815; cells[4] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 534; return *q * 2; }
    sum += f1(&cells[3], &cells[5]);
    cells[6] = cells[3] + 397;
    static int f2(int *p, int *q) { *p += 940; return *q * 2; }
    sum += f2(&cells[3], &cells[1]);
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    cells[5] = cells[7] + 345;
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 452u; }
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 275u; }
    static int f3(int *p, int *q) { *p += 342; return *q * 2; }
    sum += f3(&cells[0], &cells[0]);
    cells[4] = cells[1] + 689;
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 157u; }
    arr[0 % 6] = 374; cells[5] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
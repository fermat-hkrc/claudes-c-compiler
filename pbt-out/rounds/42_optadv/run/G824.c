static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 544u; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    static int f1(int *p, int *q) { *p += 884; return *q * 2; }
    sum += f1(&cells[5], &cells[3]);
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 501u; }
    cells[4] = cells[7] + 42;
    arr[7 % 6] = 813; cells[6] += arr[7 % 6];
    cells[3] = cells[4] + 688;
    cells[0] = cells[0] + 815;
    static int f2(int *p, int *q) { *p += 728; return *q * 2; }
    sum += f2(&cells[0], &cells[5]);
    arr[6 % 6] = 695; cells[5] += arr[6 % 6];
    static int f3(int *p, int *q) { *p += 147; return *q * 2; }
    sum += f3(&cells[0], &cells[0]);
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    cells[5] = cells[0] + 698;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 803u; }
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    cells[6] = cells[5] + 305;
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    cells[4] = cells[3] + 58;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
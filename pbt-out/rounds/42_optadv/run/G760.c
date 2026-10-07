static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[7] = cells[7] + 115;
    cells[1] = cells[4] + 467;
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    cells[4] = cells[5] + 728;
    cells[7] = cells[7] + 245;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    static int f1(int *p, int *q) { *p += 725; return *q * 2; }
    sum += f1(&cells[4], &cells[5]);
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    arr[0 % 6] = 455; cells[7] += arr[0 % 6];
    cells[2] = cells[4] + 166;
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 323u; }
    static int f2(int *p, int *q) { *p += 377; return *q * 2; }
    sum += f2(&cells[7], &cells[3]);
    cells[5] = cells[2] + 440;
    arr[0 % 6] = 41; cells[6] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
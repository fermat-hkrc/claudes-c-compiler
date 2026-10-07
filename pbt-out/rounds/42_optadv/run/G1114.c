static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    static int f1(int *p, int *q) { *p += 659; return *q * 2; }
    sum += f1(&cells[2], &cells[7]);
    static int f2(int *p, int *q) { *p += 613; return *q * 2; }
    sum += f2(&cells[6], &cells[0]);
    arr[7 % 6] = 947; cells[7] += arr[7 % 6];
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 26u; }
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    static int f3(int *p, int *q) { *p += 874; return *q * 2; }
    sum += f3(&cells[7], &cells[5]);
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 478u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[7 % 6] = 187; cells[4] += arr[7 % 6];
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    cells[7] = cells[4] + 770;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    cells[5] = cells[5] + 896;
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 677u; }
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 33u; }
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 889u; }
    arr[4 % 6] = 79; cells[0] += arr[4 % 6];
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    static int f1(int *p, int *q) { *p += 737; return *q * 2; }
    sum += f1(&cells[0], &cells[5]);
    cells[6] = cells[4] + 816;
    static int f2(int *p, int *q) { *p += 433; return *q * 2; }
    sum += f2(&cells[4], &cells[4]);
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 592u; }
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 677u; }
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 546u; }
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    arr[1 % 6] = 680; cells[3] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    cells[6] = cells[3] + 74;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
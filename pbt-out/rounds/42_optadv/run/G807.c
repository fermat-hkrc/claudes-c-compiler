static int f1(int *p, int *q) { *p += 636; return *q * 2; }
static int f2(int *p, int *q) { *p += 248; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    cells[7] = cells[3] + 674;
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 185u; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += f1(&cells[7], &cells[5]);
    sum += f2(&cells[5], &cells[3]);
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 985u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 630u; }
    cells[3] = cells[4] + 280;
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    cells[6] = cells[3] + 417;
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    cells[0] = cells[3] + 285;
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    arr[4 % 6] = 773; cells[0] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
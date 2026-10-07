static int f1(int *p, int *q) { *p += 686; return *q * 2; }
static int f2(int *p, int *q) { *p += 887; return *q * 2; }
static int f3(int *p, int *q) { *p += 996; return *q * 2; }
static int f4(int *p, int *q) { *p += 291; return *q * 2; }
static int f5(int *p, int *q) { *p += 455; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[5], &cells[6]);
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    arr[4 % 6] = 466; cells[0] += arr[4 % 6];
    sum += f2(&cells[1], &cells[2]);
    sum += f3(&cells[3], &cells[7]);
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    sum += f4(&cells[1], &cells[5]);
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 930u; }
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    cells[1] = cells[0] + 510;
    sum += f5(&cells[2], &cells[2]);
    cells[1] = cells[1] + 722;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    cells[7] = cells[1] + 711;
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
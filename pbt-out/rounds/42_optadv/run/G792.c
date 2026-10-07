static int f1(int *p, int *q) { *p += 272; return *q * 2; }
static int f2(int *p, int *q) { *p += 141; return *q * 2; }
static int f3(int *p, int *q) { *p += 592; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 438u; }
    sum += f1(&cells[5], &cells[1]);
    arr[3 % 6] = 634; cells[1] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    sum += f2(&cells[2], &cells[6]);
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    sum += f3(&cells[5], &cells[7]);
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
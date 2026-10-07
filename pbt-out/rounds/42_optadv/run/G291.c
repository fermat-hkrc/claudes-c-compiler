static int f1(int *p, int *q) { *p += 603; return *q * 2; }
static int f2(int *p, int *q) { *p += 257; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 40; cells[2] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 525u; }
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += f1(&cells[7], &cells[5]);
    sum += f2(&cells[0], &cells[1]);
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 870u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
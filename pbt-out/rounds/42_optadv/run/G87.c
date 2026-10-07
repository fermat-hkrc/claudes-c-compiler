static int f1(int *p, int *q) { *p += 19; return *q * 2; }
static int f2(int *p, int *q) { *p += 680; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 128; cells[5] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 562u; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    sum += f1(&cells[6], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[3 % 6] = 416; cells[5] += arr[3 % 6];
    cells[0] = cells[5] + 351;
    arr[5 % 6] = 202; cells[3] += arr[5 % 6];
    sum += f2(&cells[5], &cells[0]);
    arr[2 % 6] = 39; cells[7] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
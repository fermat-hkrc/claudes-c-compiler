static int f1(int *p, int *q) { *p += 870; return *q * 2; }
static int f2(int *p, int *q) { *p += 835; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 335u; }
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 377u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[6] = cells[1] + 374;
    arr[5 % 6] = 821; cells[6] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    arr[5 % 6] = 914; cells[2] += arr[5 % 6];
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 119u; }
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    arr[2 % 6] = 22; cells[7] += arr[2 % 6];
    arr[4 % 6] = 660; cells[1] += arr[4 % 6];
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    arr[2 % 6] = 851; cells[6] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f1(&cells[0], &cells[3]);
    sum += f2(&cells[2], &cells[3]);
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
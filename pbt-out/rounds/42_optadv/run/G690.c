static int f1(int *p, int *q) { *p += 560; return *q * 2; }
static int f2(int *p, int *q) { *p += 779; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 723; cells[3] += arr[6 % 6];
    sum += f1(&cells[7], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    arr[2 % 6] = 384; cells[2] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    arr[0 % 6] = 435; cells[5] += arr[0 % 6];
    arr[3 % 6] = 287; cells[6] += arr[3 % 6];
    cells[6] = cells[2] + 505;
    sum += f2(&cells[1], &cells[3]);
    cells[3] = cells[7] + 539;
    arr[5 % 6] = 431; cells[1] += arr[5 % 6];
    cells[5] = cells[3] + 150;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 2u; }
    arr[6 % 6] = 35; cells[5] += arr[6 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 449; return *q * 2; }
static int f2(int *p, int *q) { *p += 276; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[2] + 710;
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 928u; }
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    cells[3] = cells[4] + 885;
    cells[0] = cells[0] + 4;
    cells[2] = cells[6] + 783;
    cells[4] = cells[0] + 613;
    sum += f1(&cells[4], &cells[5]);
    arr[2 % 6] = 746; cells[1] += arr[2 % 6];
    arr[0 % 6] = 425; cells[7] += arr[0 % 6];
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    cells[5] = cells[1] + 195;
    sum += f2(&cells[1], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 397u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[5] = cells[0] + 258;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
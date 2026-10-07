static int f1(int *p, int *q) { *p += 199; return *q * 2; }
static int f2(int *p, int *q) { *p += 111; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    sum += f1(&cells[7], &cells[7]);
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    arr[6 % 6] = 12; cells[0] += arr[6 % 6];
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 869u; }
    arr[1 % 6] = 348; cells[1] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[2] = cells[1] + 592;
    cells[4] = cells[7] + 375;
    arr[2 % 6] = 135; cells[7] += arr[2 % 6];
    sum += f2(&cells[1], &cells[2]);
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 754u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 351u; }
    cells[2] = cells[7] + 467;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
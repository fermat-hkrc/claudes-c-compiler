static int f1(int *p, int *q) { *p += 17; return *q * 2; }
static int f2(int *p, int *q) { *p += 96; return *q * 2; }
static int f3(int *p, int *q) { *p += 808; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    sum += f1(&cells[3], &cells[3]);
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 548u; }
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 657u; }
    sum += f2(&cells[2], &cells[1]);
    cells[3] = cells[2] + 553;
    sum += f3(&cells[5], &cells[0]);
    arr[6 % 6] = 923; cells[2] += arr[6 % 6];
    cells[0] = cells[0] + 193;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 890u; }
    cells[5] = cells[2] + 441;
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 636u; }
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 259u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[3] = cells[6] + 701;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
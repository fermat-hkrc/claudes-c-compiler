static int f1(int *p, int *q) { *p += 511; return *q * 2; }
static int f2(int *p, int *q) { *p += 230; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    arr[4 % 6] = 427; cells[1] += arr[4 % 6];
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 318u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 275u; }
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 609u; }
    sum += f1(&cells[0], &cells[5]);
    sum += f2(&cells[6], &cells[2]);
    cells[4] = cells[0] + 340;
    cells[5] = cells[1] + 310;
    arr[7 % 6] = 754; cells[5] += arr[7 % 6];
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 691u; }
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    cells[2] = cells[0] + 121;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
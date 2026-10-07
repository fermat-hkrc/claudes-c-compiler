static int f1(int *p, int *q) { *p += 102; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 817u; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    cells[4] = cells[7] + 254;
    arr[0 % 6] = 539; cells[1] += arr[0 % 6];
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 992u; }
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 0u; }
    cells[6] = cells[5] + 497;
    cells[1] = cells[0] + 316;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    arr[3 % 6] = 359; cells[3] += arr[3 % 6];
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 613u; }
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 567u; }
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 234u; }
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    sum += f1(&cells[1], &cells[0]);
    cells[6] = cells[7] + 542;
    arr[6 % 6] = 994; cells[7] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
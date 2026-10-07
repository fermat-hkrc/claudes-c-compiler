static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 511; cells[3] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    arr[2 % 6] = 55; cells[5] += arr[2 % 6];
    cells[1] = cells[6] + 533;
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 71u; }
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 694u; }
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    arr[7 % 6] = 662; cells[4] += arr[7 % 6];
    static int f1(int *p, int *q) { *p += 567; return *q * 2; }
    sum += f1(&cells[1], &cells[2]);
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 549u; }
    arr[1 % 6] = 934; cells[4] += arr[1 % 6];
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 16u; }
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    cells[0] = cells[4] + 513;
    arr[3 % 6] = 531; cells[0] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[1 % 6] = 612; cells[4] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 938u; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    arr[0 % 6] = 253; cells[0] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 480; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    cells[0] = cells[7] + 153;
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 442u; }
    sum += f1(&cells[4], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[5 % 6] = 963; cells[3] += arr[5 % 6];
    arr[5 % 6] = 408; cells[2] += arr[5 % 6];
    arr[5 % 6] = 49; cells[2] += arr[5 % 6];
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 572u; }
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    cells[0] = cells[5] + 453;
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 48u; }
    cells[1] = cells[4] + 212;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
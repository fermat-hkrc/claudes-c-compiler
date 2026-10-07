static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[3] + 884;
    arr[7 % 6] = 585; cells[1] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    arr[5 % 6] = 903; cells[6] += arr[5 % 6];
    cells[1] = cells[6] + 596;
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 366u; }
    arr[4 % 6] = 744; cells[3] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[1] = cells[6] + 672;
    cells[4] = cells[6] + 288;
    arr[0 % 6] = 30; cells[6] += arr[0 % 6];
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 77u; }
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 881u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
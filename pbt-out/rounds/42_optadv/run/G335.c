static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    cells[1] = cells[3] + 632;
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 522u; }
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    arr[7 % 6] = 964; cells[1] += arr[7 % 6];
    cells[2] = cells[2] + 175;
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 384u; }
    arr[3 % 6] = 301; cells[1] += arr[3 % 6];
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    arr[1 % 6] = 747; cells[7] += arr[1 % 6];
    cells[3] = cells[6] + 538;
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    arr[3 % 6] = 332; cells[1] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
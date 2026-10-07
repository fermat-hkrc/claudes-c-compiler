static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 197u; }
    arr[1 % 6] = 209; cells[6] += arr[1 % 6];
    cells[7] = cells[7] + 321;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[5 % 6] = 956; cells[0] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    arr[7 % 6] = 68; cells[3] += arr[7 % 6];
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    cells[1] = cells[2] + 656;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[5 % 6] = 293; cells[6] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[2 % 6] = 395; cells[4] += arr[2 % 6];
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 302u; }
    cells[4] = cells[3] + 638;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    cells[5] = cells[0] + 171;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    static int f1(int *p, int *q) { *p += 359; return *q * 2; }
    sum += f1(&cells[5], &cells[5]);
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
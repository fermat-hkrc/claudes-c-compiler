static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[6] + 236;
    arr[4 % 6] = 597; cells[3] += arr[4 % 6];
    cells[6] = cells[4] + 29;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    cells[3] = cells[7] + 625;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 597u; }
    static int f1(int *p, int *q) { *p += 548; return *q * 2; }
    sum += f1(&cells[6], &cells[1]);
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    arr[2 % 6] = 20; cells[0] += arr[2 % 6];
    cells[3] = cells[2] + 197;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[3 % 6] = 695; cells[0] += arr[3 % 6];
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 922u; }
    cells[5] = cells[4] + 569;
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
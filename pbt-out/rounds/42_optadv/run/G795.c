static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[3] = cells[0] + 929;
    static int f1(int *p, int *q) { *p += 582; return *q * 2; }
    sum += f1(&cells[5], &cells[7]);
    arr[1 % 6] = 842; cells[2] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    cells[7] = cells[6] + 500;
    cells[3] = cells[3] + 331;
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 494u; }
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    static int f2(int *p, int *q) { *p += 397; return *q * 2; }
    sum += f2(&cells[6], &cells[0]);
    arr[3 % 6] = 176; cells[7] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 195; cells[3] += arr[4 % 6];
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    static int f1(int *p, int *q) { *p += 893; return *q * 2; }
    sum += f1(&cells[6], &cells[5]);
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    arr[6 % 6] = 680; cells[4] += arr[6 % 6];
    arr[6 % 6] = 525; cells[6] += arr[6 % 6];
    arr[6 % 6] = 90; cells[0] += arr[6 % 6];
    arr[4 % 6] = 287; cells[2] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    static int f2(int *p, int *q) { *p += 229; return *q * 2; }
    sum += f2(&cells[0], &cells[6]);
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    static int f3(int *p, int *q) { *p += 399; return *q * 2; }
    sum += f3(&cells[6], &cells[6]);
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[2] = cells[3] + 837;
    arr[4 % 6] = 115; cells[7] += arr[4 % 6];
    arr[4 % 6] = 498; cells[6] += arr[4 % 6];
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 205u; }
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 620u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
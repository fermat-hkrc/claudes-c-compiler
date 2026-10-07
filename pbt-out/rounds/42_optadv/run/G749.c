static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    arr[7 % 6] = 739; cells[6] += arr[7 % 6];
    static int f1(int *p, int *q) { *p += 539; return *q * 2; }
    sum += f1(&cells[0], &cells[3]);
    cells[2] = cells[1] + 484;
    static int f2(int *p, int *q) { *p += 329; return *q * 2; }
    sum += f2(&cells[1], &cells[4]);
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    arr[0 % 6] = 527; cells[0] += arr[0 % 6];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    arr[2 % 6] = 729; cells[6] += arr[2 % 6];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 538u; }
    static int f3(int *p, int *q) { *p += 365; return *q * 2; }
    sum += f3(&cells[6], &cells[6]);
    arr[3 % 6] = 614; cells[0] += arr[3 % 6];
    arr[7 % 6] = 982; cells[6] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[1 % 6] = 580; cells[1] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
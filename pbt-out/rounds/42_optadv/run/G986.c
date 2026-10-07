static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    cells[4] = cells[4] + 324;
    cells[2] = cells[6] + 800;
    cells[3] = cells[0] + 246;
    cells[7] = cells[2] + 900;
    static int f1(int *p, int *q) { *p += 681; return *q * 2; }
    sum += f1(&cells[0], &cells[7]);
    arr[7 % 6] = 444; cells[4] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    static int f2(int *p, int *q) { *p += 677; return *q * 2; }
    sum += f2(&cells[0], &cells[6]);
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    static int f3(int *p, int *q) { *p += 754; return *q * 2; }
    sum += f3(&cells[4], &cells[5]);
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 522u; }
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    arr[1 % 6] = 197; cells[7] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
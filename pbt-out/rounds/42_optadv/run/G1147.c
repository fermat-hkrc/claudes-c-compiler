static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 592; cells[1] += arr[0 % 6];
    static int f1(int *p, int *q) { *p += 74; return *q * 2; }
    sum += f1(&cells[0], &cells[3]);
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 730u; }
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    cells[3] = cells[6] + 188;
    static int f2(int *p, int *q) { *p += 7; return *q * 2; }
    sum += f2(&cells[3], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[2 % 6] = 128; cells[4] += arr[2 % 6];
    static int f3(int *p, int *q) { *p += 784; return *q * 2; }
    sum += f3(&cells[6], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[1] = cells[1] + 568;
    static int f4(int *p, int *q) { *p += 967; return *q * 2; }
    sum += f4(&cells[3], &cells[6]);
    arr[2 % 6] = 361; cells[3] += arr[2 % 6];
    arr[5 % 6] = 218; cells[1] += arr[5 % 6];
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 25u; }
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    arr[3 % 6] = 957; cells[7] += arr[3 % 6];
    cells[4] = cells[5] + 498;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
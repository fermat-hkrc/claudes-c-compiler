static int f1(int *p, int *q) { *p += 437; return *q * 2; }
static int f2(int *p, int *q) { *p += 77; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 889; cells[3] += arr[2 % 6];
    sum += f1(&cells[5], &cells[4]);
    arr[4 % 6] = 784; cells[3] += arr[4 % 6];
    arr[2 % 6] = 574; cells[6] += arr[2 % 6];
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    arr[1 % 6] = 806; cells[5] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[2] = cells[5] + 369;
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 214u; }
    cells[5] = cells[7] + 320;
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 16u; }
    sum += f2(&cells[6], &cells[7]);
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
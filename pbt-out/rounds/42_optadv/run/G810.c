static int f1(int *p, int *q) { *p += 539; return *q * 2; }
static int f2(int *p, int *q) { *p += 651; return *q * 2; }
static int f3(int *p, int *q) { *p += 985; return *q * 2; }
static int f4(int *p, int *q) { *p += 710; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[7], &cells[2]);
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 937u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f2(&cells[4], &cells[7]);
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    arr[6 % 6] = 852; cells[5] += arr[6 % 6];
    arr[5 % 6] = 615; cells[6] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    arr[2 % 6] = 81; cells[1] += arr[2 % 6];
    sum += f3(&cells[2], &cells[6]);
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 387u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f4(&cells[7], &cells[7]);
    cells[4] = cells[4] + 784;
    cells[7] = cells[4] + 311;
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 921u; }
    arr[4 % 6] = 766; cells[4] += arr[4 % 6];
    arr[3 % 6] = 446; cells[5] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
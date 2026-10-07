static int f1(int *p, int *q) { *p += 740; return *q * 2; }
static int f2(int *p, int *q) { *p += 196; return *q * 2; }
static int f3(int *p, int *q) { *p += 988; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    arr[2 % 6] = 196; cells[1] += arr[2 % 6];
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 188u; }
    arr[1 % 6] = 897; cells[6] += arr[1 % 6];
    sum += f1(&cells[5], &cells[7]);
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 636u; }
    sum += f2(&cells[3], &cells[4]);
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[2 % 6] = 524; cells[7] += arr[2 % 6];
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 400u; }
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 325u; }
    sum += f3(&cells[0], &cells[6]);
    cells[2] = cells[0] + 725;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 245; return *q * 2; }
static int f2(int *p, int *q) { *p += 43; return *q * 2; }
static int f3(int *p, int *q) { *p += 724; return *q * 2; }
static int f4(int *p, int *q) { *p += 878; return *q * 2; }
static int f5(int *p, int *q) { *p += 852; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 756u; }
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 995u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 61u; }
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 375u; }
    arr[2 % 6] = 487; cells[2] += arr[2 % 6];
    cells[1] = cells[3] + 608;
    sum += f1(&cells[5], &cells[4]);
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[7 % 6] = 581; cells[3] += arr[7 % 6];
    cells[7] = cells[1] + 243;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f2(&cells[6], &cells[7]);
    sum += f3(&cells[6], &cells[6]);
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    sum += f4(&cells[6], &cells[6]);
    sum += f5(&cells[2], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
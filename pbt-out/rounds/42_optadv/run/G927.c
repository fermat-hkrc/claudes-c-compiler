static int f1(int *p, int *q) { *p += 954; return *q * 2; }
static int f2(int *p, int *q) { *p += 596; return *q * 2; }
static int f3(int *p, int *q) { *p += 928; return *q * 2; }
static int f4(int *p, int *q) { *p += 18; return *q * 2; }
static int f5(int *p, int *q) { *p += 534; return *q * 2; }
static int f6(int *p, int *q) { *p += 809; return *q * 2; }
static int f7(int *p, int *q) { *p += 90; return *q * 2; }
static int f8(int *p, int *q) { *p += 705; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[6]);
    cells[5] = cells[5] + 920;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f2(&cells[6], &cells[0]);
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    sum += f3(&cells[7], &cells[3]);
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 5u; }
    arr[1 % 6] = 597; cells[7] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    sum += f4(&cells[3], &cells[3]);
    sum += f5(&cells[4], &cells[7]);
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 405u; }
    sum += f6(&cells[6], &cells[6]);
    sum += f7(&cells[0], &cells[7]);
    arr[5 % 6] = 130; cells[5] += arr[5 % 6];
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 634u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f8(&cells[2], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
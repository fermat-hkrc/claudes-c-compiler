static int f1(int *p, int *q) { *p += 14; return *q * 2; }
static int f2(int *p, int *q) { *p += 213; return *q * 2; }
static int f3(int *p, int *q) { *p += 721; return *q * 2; }
static int f4(int *p, int *q) { *p += 744; return *q * 2; }
static int f5(int *p, int *q) { *p += 52; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[2] + 248;
    arr[0 % 6] = 681; cells[7] += arr[0 % 6];
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 133u; }
    sum += f1(&cells[5], &cells[6]);
    arr[3 % 6] = 938; cells[4] += arr[3 % 6];
    cells[0] = cells[4] + 868;
    sum += f2(&cells[7], &cells[3]);
    sum += f3(&cells[2], &cells[6]);
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f4(&cells[5], &cells[1]);
    sum += f5(&cells[3], &cells[3]);
    cells[0] = cells[0] + 902;
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 499u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 629u; }
    cells[2] = cells[7] + 207;
    cells[5] = cells[1] + 744;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    cells[7] = cells[6] + 642;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
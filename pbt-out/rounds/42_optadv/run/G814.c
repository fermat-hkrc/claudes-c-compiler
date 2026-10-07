static int f1(int *p, int *q) { *p += 867; return *q * 2; }
static int f2(int *p, int *q) { *p += 528; return *q * 2; }
static int f3(int *p, int *q) { *p += 397; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    cells[3] = cells[5] + 892;
    sum += f1(&cells[2], &cells[3]);
    sum += f2(&cells[1], &cells[5]);
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 752u; }
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 574u; }
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 417u; }
    sum += f3(&cells[1], &cells[4]);
    arr[6 % 6] = 331; cells[1] += arr[6 % 6];
    arr[0 % 6] = 532; cells[0] += arr[0 % 6];
    cells[2] = cells[3] + 524;
    cells[7] = cells[3] + 704;
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 674u; }
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    cells[6] = cells[6] + 880;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 836; return *q * 2; }
static int f2(int *p, int *q) { *p += 118; return *q * 2; }
static int f3(int *p, int *q) { *p += 781; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[5]);
    cells[2] = cells[4] + 395;
    arr[4 % 6] = 77; cells[3] += arr[4 % 6];
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    arr[3 % 6] = 345; cells[1] += arr[3 % 6];
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 943u; }
    cells[6] = cells[6] + 210;
    sum += f2(&cells[6], &cells[3]);
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    cells[3] = cells[1] + 287;
    cells[2] = cells[5] + 427;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[4] = cells[2] + 811;
    sum += f3(&cells[4], &cells[3]);
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
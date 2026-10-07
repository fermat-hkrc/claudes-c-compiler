static int f1(int *p, int *q) { *p += 965; return *q * 2; }
static int f2(int *p, int *q) { *p += 93; return *q * 2; }
static int f3(int *p, int *q) { *p += 265; return *q * 2; }
static int f4(int *p, int *q) { *p += 710; return *q * 2; }
static int f5(int *p, int *q) { *p += 786; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    sum += f1(&cells[3], &cells[4]);
    sum += f2(&cells[0], &cells[6]);
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[5] = cells[4] + 785;
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    sum += f3(&cells[3], &cells[6]);
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += f4(&cells[0], &cells[1]);
    arr[4 % 6] = 768; cells[4] += arr[4 % 6];
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 198u; }
    cells[2] = cells[6] + 770;
    arr[2 % 6] = 613; cells[6] += arr[2 % 6];
    sum += f5(&cells[4], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
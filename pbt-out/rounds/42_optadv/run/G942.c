static int f1(int *p, int *q) { *p += 538; return *q * 2; }
static int f2(int *p, int *q) { *p += 311; return *q * 2; }
static int f3(int *p, int *q) { *p += 910; return *q * 2; }
static int f4(int *p, int *q) { *p += 533; return *q * 2; }
static int f5(int *p, int *q) { *p += 200; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 702; cells[2] += arr[4 % 6];
    sum += f1(&cells[7], &cells[7]);
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    cells[1] = cells[1] + 771;
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    sum += f2(&cells[5], &cells[5]);
    arr[1 % 6] = 548; cells[1] += arr[1 % 6];
    cells[7] = cells[7] + 165;
    sum += f3(&cells[6], &cells[6]);
    sum += f4(&cells[5], &cells[4]);
    sum += f5(&cells[0], &cells[2]);
    arr[0 % 6] = 562; cells[1] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
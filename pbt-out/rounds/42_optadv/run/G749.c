static int f1(int *p, int *q) { *p += 633; return *q * 2; }
static int f2(int *p, int *q) { *p += 584; return *q * 2; }
static int f3(int *p, int *q) { *p += 736; return *q * 2; }
static int f4(int *p, int *q) { *p += 562; return *q * 2; }
static int f5(int *p, int *q) { *p += 242; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[7] + 694;
    cells[2] = cells[5] + 949;
    sum += f1(&cells[5], &cells[6]);
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    sum += f2(&cells[6], &cells[5]);
    sum += f3(&cells[0], &cells[2]);
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    arr[6 % 6] = 16; cells[6] += arr[6 % 6];
    cells[2] = cells[6] + 759;
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 789u; }
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 410u; }
    sum += f4(&cells[6], &cells[2]);
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    arr[4 % 6] = 732; cells[3] += arr[4 % 6];
    arr[7 % 6] = 765; cells[3] += arr[7 % 6];
    arr[1 % 6] = 580; cells[0] += arr[1 % 6];
    cells[2] = cells[4] + 666;
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 278u; }
    sum += f5(&cells[1], &cells[2]);
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
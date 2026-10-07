static int f1(int *p, int *q) { *p += 607; return *q * 2; }
static int f2(int *p, int *q) { *p += 245; return *q * 2; }
static int f3(int *p, int *q) { *p += 663; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[3] + 119;
    cells[2] = cells[4] + 644;
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    sum += f1(&cells[2], &cells[7]);
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    sum += f2(&cells[6], &cells[1]);
    cells[6] = cells[0] + 766;
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    cells[1] = cells[0] + 351;
    sum += f3(&cells[1], &cells[7]);
    cells[3] = cells[0] + 78;
    cells[0] = cells[5] + 15;
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
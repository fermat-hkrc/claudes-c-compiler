static int f1(int *p, int *q) { *p += 927; return *q * 2; }
static int f2(int *p, int *q) { *p += 602; return *q * 2; }
static int f3(int *p, int *q) { *p += 648; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    sum += f1(&cells[6], &cells[5]);
    sum += f2(&cells[7], &cells[0]);
    cells[4] = cells[0] + 377;
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    sum += f3(&cells[5], &cells[2]);
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
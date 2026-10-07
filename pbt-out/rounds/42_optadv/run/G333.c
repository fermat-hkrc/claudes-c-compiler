static int f1(int *p, int *q) { *p += 448; return *q * 2; }
static int f2(int *p, int *q) { *p += 973; return *q * 2; }
static int f3(int *p, int *q) { *p += 425; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[3], &cells[6]);
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    cells[6] = cells[1] + 620;
    sum += f2(&cells[1], &cells[2]);
    sum += f3(&cells[5], &cells[5]);
    cells[2] = cells[0] + 956;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
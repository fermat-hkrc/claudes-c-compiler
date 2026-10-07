static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 950; cells[1] += arr[1 % 6];
    cells[5] = cells[5] + 580;
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f1(int *p, int *q) { *p += 138; return *q * 2; }
    sum += f1(&cells[7], &cells[5]);
    static int f2(int *p, int *q) { *p += 525; return *q * 2; }
    sum += f2(&cells[1], &cells[1]);
    cells[1] = cells[6] + 692;
    static int f3(int *p, int *q) { *p += 834; return *q * 2; }
    sum += f3(&cells[5], &cells[2]);
    static int f4(int *p, int *q) { *p += 852; return *q * 2; }
    sum += f4(&cells[1], &cells[0]);
    static int f5(int *p, int *q) { *p += 930; return *q * 2; }
    sum += f5(&cells[1], &cells[7]);
    cells[7] = cells[5] + 648;
    static int f6(int *p, int *q) { *p += 124; return *q * 2; }
    sum += f6(&cells[2], &cells[2]);
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[1] = cells[1] + 904;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
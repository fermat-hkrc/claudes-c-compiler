static int f1(int *p, int *q) { *p += 649; return *q * 2; }
static int f2(int *p, int *q) { *p += 998; return *q * 2; }
static int f3(int *p, int *q) { *p += 213; return *q * 2; }
static int f4(int *p, int *q) { *p += 288; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[4], &cells[7]);
    sum += f2(&cells[1], &cells[7]);
    arr[5 % 6] = 71; cells[1] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[0 % 6] = 234; cells[7] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    arr[2 % 6] = 738; cells[6] += arr[2 % 6];
    cells[6] = cells[5] + 61;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    cells[2] = cells[3] + 324;
    cells[3] = cells[6] + 670;
    sum += f3(&cells[1], &cells[4]);
    sum += f4(&cells[7], &cells[0]);
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
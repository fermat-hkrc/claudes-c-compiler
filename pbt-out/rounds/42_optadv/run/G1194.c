static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    arr[1 % 6] = 240; cells[3] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    static int f1(int *p, int *q) { *p += 859; return *q * 2; }
    sum += f1(&cells[7], &cells[6]);
    cells[0] = cells[0] + 258;
    static int f2(int *p, int *q) { *p += 515; return *q * 2; }
    sum += f2(&cells[5], &cells[6]);
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    arr[1 % 6] = 681; cells[2] += arr[1 % 6];
    arr[5 % 6] = 657; cells[4] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
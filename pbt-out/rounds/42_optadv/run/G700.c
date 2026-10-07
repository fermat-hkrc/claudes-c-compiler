static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 716; cells[1] += arr[1 % 6];
    arr[6 % 6] = 783; cells[4] += arr[6 % 6];
    static int f1(int *p, int *q) { *p += 525; return *q * 2; }
    sum += f1(&cells[7], &cells[4]);
    static int f2(int *p, int *q) { *p += 288; return *q * 2; }
    sum += f2(&cells[5], &cells[0]);
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    static int f3(int *p, int *q) { *p += 119; return *q * 2; }
    sum += f3(&cells[5], &cells[1]);
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    cells[0] = cells[2] + 776;
    static int f4(int *p, int *q) { *p += 71; return *q * 2; }
    sum += f4(&cells[0], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
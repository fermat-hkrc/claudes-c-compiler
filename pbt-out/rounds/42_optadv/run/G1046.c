static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 721; cells[0] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 898; return *q * 2; }
    sum += f1(&cells[5], &cells[6]);
    cells[1] = cells[0] + 716;
    static int f2(int *p, int *q) { *p += 677; return *q * 2; }
    sum += f2(&cells[4], &cells[7]);
    cells[6] = cells[1] + 148;
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
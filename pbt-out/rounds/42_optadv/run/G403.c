static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    arr[3 % 6] = 617; cells[5] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 451; return *q * 2; }
    sum += f1(&cells[5], &cells[4]);
    cells[3] = cells[1] + 411;
    static int f2(int *p, int *q) { *p += 496; return *q * 2; }
    sum += f2(&cells[5], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
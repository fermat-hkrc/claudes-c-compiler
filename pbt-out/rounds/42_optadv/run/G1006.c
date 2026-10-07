static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    static int f1(int *p, int *q) { *p += 841; return *q * 2; }
    sum += f1(&cells[0], &cells[4]);
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    cells[5] = cells[2] + 459;
    cells[0] = cells[5] + 9;
    cells[5] = cells[1] + 507;
    static int f2(int *p, int *q) { *p += 446; return *q * 2; }
    sum += f2(&cells[6], &cells[2]);
    arr[2 % 6] = 306; cells[5] += arr[2 % 6];
    static int f3(int *p, int *q) { *p += 628; return *q * 2; }
    sum += f3(&cells[0], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
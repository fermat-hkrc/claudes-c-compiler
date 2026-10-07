static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 942; return *q * 2; }
    sum += f1(&cells[1], &cells[3]);
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    cells[0] = cells[2] + 405;
    arr[2 % 6] = 651; cells[3] += arr[2 % 6];
    arr[6 % 6] = 456; cells[1] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    arr[3 % 6] = 572; cells[5] += arr[3 % 6];
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    cells[3] = cells[1] + 166;
    static int f2(int *p, int *q) { *p += 813; return *q * 2; }
    sum += f2(&cells[5], &cells[1]);
    cells[3] = cells[3] + 85;
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
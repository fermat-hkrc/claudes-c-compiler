static int f1(int *p, int *q) { *p += 816; return *q * 2; }
static int f2(int *p, int *q) { *p += 879; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[7]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 942u; }
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    cells[0] = cells[7] + 308;
    sum += f1(&cells[0], &cells[6]);
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    cells[3] = cells[4] + 201;
    arr[0 % 6] = 835; cells[1] += arr[0 % 6];
    arr[7 % 6] = 5; cells[7] += arr[7 % 6];
    sum += f2(&cells[6], &cells[5]);
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
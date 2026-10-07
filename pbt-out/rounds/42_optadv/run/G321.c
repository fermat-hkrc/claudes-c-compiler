static int f1(int *p, int *q) { *p += 35; return *q * 2; }
static int f2(int *p, int *q) { *p += 743; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    cells[5] = cells[3] + 881;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 261u; }
    arr[0 % 6] = 63; cells[2] += arr[0 % 6];
    sum += f1(&cells[5], &cells[6]);
    sum += f2(&cells[0], &cells[3]);
    arr[0 % 6] = 162; cells[7] += arr[0 % 6];
    arr[7 % 6] = 799; cells[2] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
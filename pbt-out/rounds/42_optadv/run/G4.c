static int f1(int *p, int *q) { *p += 425; return *q * 2; }
static int f2(int *p, int *q) { *p += 795; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[6] + 273;
    arr[0 % 6] = 96; cells[1] += arr[0 % 6];
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 871u; }
    sum += f1(&cells[6], &cells[6]);
    sum += f2(&cells[7], &cells[5]);
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    cells[7] = cells[4] + 102;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 285; return *q * 2; }
static int f2(int *p, int *q) { *p += 654; return *q * 2; }
static int f3(int *p, int *q) { *p += 329; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 656; cells[4] += arr[6 % 6];
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 588u; }
    sum += f1(&cells[1], &cells[5]);
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    sum += f2(&cells[7], &cells[2]);
    cells[7] = cells[6] + 192;
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 789u; }
    sum += f3(&cells[3], &cells[1]);
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 523u; }
    arr[3 % 6] = 372; cells[6] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
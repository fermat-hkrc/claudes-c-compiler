static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 885; cells[6] += arr[5 % 6];
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 359u; }
    cells[5] = cells[1] + 463;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    static int f1(int *p, int *q) { *p += 839; return *q * 2; }
    sum += f1(&cells[0], &cells[0]);
    cells[2] = cells[6] + 438;
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 831u; }
    static int f2(int *p, int *q) { *p += 757; return *q * 2; }
    sum += f2(&cells[6], &cells[3]);
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
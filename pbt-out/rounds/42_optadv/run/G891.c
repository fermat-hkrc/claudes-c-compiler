static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[4 % 6] = 861; cells[4] += arr[4 % 6];
    cells[7] = cells[1] + 810;
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 27u; }
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 735u; }
    cells[5] = cells[5] + 5;
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    arr[0 % 6] = 641; cells[0] += arr[0 % 6];
    cells[3] = cells[7] + 125;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 621u; }
    arr[1 % 6] = 863; cells[0] += arr[1 % 6];
    arr[3 % 6] = 517; cells[6] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    static int f1(int *p, int *q) { *p += 272; return *q * 2; }
    sum += f1(&cells[5], &cells[1]);
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 966u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 32u; }
    arr[4 % 6] = 848; cells[3] += arr[4 % 6];
    cells[7] = cells[1] + 736;
    arr[7 % 6] = 287; cells[4] += arr[7 % 6];
    static int f1(int *p, int *q) { *p += 160; return *q * 2; }
    sum += f1(&cells[1], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
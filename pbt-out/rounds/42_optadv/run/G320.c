static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[0] + 228;
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 760u; }
    static int f1(int *p, int *q) { *p += 776; return *q * 2; }
    sum += f1(&cells[5], &cells[5]);
    static int f2(int *p, int *q) { *p += 515; return *q * 2; }
    sum += f2(&cells[4], &cells[3]);
    cells[6] = cells[6] + 501;
    cells[0] = cells[0] + 26;
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
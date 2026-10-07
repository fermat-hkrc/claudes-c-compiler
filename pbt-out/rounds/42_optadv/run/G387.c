static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 952; return *q * 2; }
    sum += f1(&cells[6], &cells[5]);
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 810u; }
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    cells[0] = cells[5] + 487;
    arr[0 % 6] = 166; cells[1] += arr[0 % 6];
    static int f2(int *p, int *q) { *p += 916; return *q * 2; }
    sum += f2(&cells[4], &cells[4]);
    cells[3] = cells[7] + 321;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
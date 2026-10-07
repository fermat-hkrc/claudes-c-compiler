static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    arr[7 % 6] = 725; cells[3] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    cells[7] = cells[4] + 193;
    cells[7] = cells[0] + 454;
    static int f1(int *p, int *q) { *p += 732; return *q * 2; }
    sum += f1(&cells[4], &cells[7]);
    arr[1 % 6] = 128; cells[1] += arr[1 % 6];
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 994u; }
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    cells[3] = cells[2] + 553;
    arr[2 % 6] = 731; cells[1] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
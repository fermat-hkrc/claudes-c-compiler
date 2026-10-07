static int f1(int *p, int *q) { *p += 92; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 623u; }
    cells[7] = cells[4] + 806;
    cells[5] = cells[4] + 632;
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 753u; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 796u; }
    cells[6] = cells[2] + 470;
    cells[7] = cells[2] + 227;
    sum += f1(&cells[2], &cells[4]);
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
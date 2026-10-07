static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 573; return *q * 2; }
    sum += f1(&cells[7], &cells[6]);
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    static int f2(int *p, int *q) { *p += 20; return *q * 2; }
    sum += f2(&cells[0], &cells[2]);
    cells[3] = cells[2] + 20;
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 110u; }
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 153u; }
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
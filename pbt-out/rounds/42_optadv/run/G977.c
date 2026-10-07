static int f1(int *p, int *q) { *p += 251; return *q * 2; }
static int f2(int *p, int *q) { *p += 33; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    cells[0] = cells[3] + 759;
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 489u; }
    sum += f1(&cells[3], &cells[1]);
    sum += f2(&cells[2], &cells[5]);
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 781u; }
    arr[2 % 6] = 874; cells[6] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 536u; }
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 619u; }
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 37u; }
    arr[3 % 6] = 211; cells[1] += arr[3 % 6];
    arr[1 % 6] = 585; cells[5] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
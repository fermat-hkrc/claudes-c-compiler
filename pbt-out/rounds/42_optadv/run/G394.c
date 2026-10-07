static int f1(int *p, int *q) { *p += 109; return *q * 2; }
static int f2(int *p, int *q) { *p += 201; return *q * 2; }
static int f3(int *p, int *q) { *p += 358; return *q * 2; }
static int f4(int *p, int *q) { *p += 925; return *q * 2; }
static int f5(int *p, int *q) { *p += 489; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 149u; }
    sum += f1(&cells[7], &cells[5]);
    sum += f2(&cells[3], &cells[0]);
    sum += f3(&cells[7], &cells[2]);
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 998u; }
    sum += f4(&cells[3], &cells[3]);
    sum += f5(&cells[0], &cells[0]);
    cells[3] = cells[0] + 468;
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
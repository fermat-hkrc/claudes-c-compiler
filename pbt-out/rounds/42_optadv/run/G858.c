static int f1(int *p, int *q) { *p += 513; return *q * 2; }
static int f2(int *p, int *q) { *p += 370; return *q * 2; }
static int f3(int *p, int *q) { *p += 416; return *q * 2; }
static int f4(int *p, int *q) { *p += 851; return *q * 2; }
static int f5(int *p, int *q) { *p += 306; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[4]);
    cells[0] = cells[7] + 943;
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 460u; }
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    sum += f2(&cells[2], &cells[0]);
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    sum += f3(&cells[3], &cells[5]);
    sum += f4(&cells[4], &cells[2]);
    sum += f5(&cells[3], &cells[5]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
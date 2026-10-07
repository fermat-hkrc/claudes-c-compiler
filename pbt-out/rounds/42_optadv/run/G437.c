static int f1(int *p, int *q) { *p += 348; return *q * 2; }
static int f2(int *p, int *q) { *p += 605; return *q * 2; }
static int f3(int *p, int *q) { *p += 769; return *q * 2; }
static int f4(int *p, int *q) { *p += 85; return *q * 2; }
static int f5(int *p, int *q) { *p += 799; return *q * 2; }
static int f6(int *p, int *q) { *p += 833; return *q * 2; }
static int f7(int *p, int *q) { *p += 332; return *q * 2; }
static int f8(int *p, int *q) { *p += 932; return *q * 2; }
static int f9(int *p, int *q) { *p += 328; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    sum += f1(&cells[1], &cells[0]);
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 250u; }
    cells[4] = cells[6] + 623;
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    sum += f2(&cells[6], &cells[0]);
    sum += f3(&cells[4], &cells[5]);
    arr[5 % 6] = 778; cells[7] += arr[5 % 6];
    sum += f4(&cells[2], &cells[3]);
    sum += f5(&cells[1], &cells[6]);
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    arr[0 % 6] = 681; cells[0] += arr[0 % 6];
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 890u; }
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 913u; }
    arr[2 % 6] = 379; cells[2] += arr[2 % 6];
    sum += f6(&cells[0], &cells[4]);
    sum += f7(&cells[0], &cells[3]);
    sum += f8(&cells[2], &cells[5]);
    sum += f9(&cells[0], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
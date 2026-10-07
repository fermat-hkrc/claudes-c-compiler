static int f1(int *p, int *q) { *p += 204; return *q * 2; }
static int f2(int *p, int *q) { *p += 946; return *q * 2; }
static int f3(int *p, int *q) { *p += 630; return *q * 2; }
static int f4(int *p, int *q) { *p += 678; return *q * 2; }
static int f5(int *p, int *q) { *p += 655; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    cells[6] = cells[2] + 52;
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 652u; }
    sum += f1(&cells[6], &cells[4]);
    sum += f2(&cells[0], &cells[2]);
    sum += f3(&cells[7], &cells[7]);
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 913u; }
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    cells[3] = cells[4] + 540;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    cells[5] = cells[4] + 315;
    cells[4] = cells[3] + 918;
    sum += f4(&cells[5], &cells[2]);
    sum += f5(&cells[3], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
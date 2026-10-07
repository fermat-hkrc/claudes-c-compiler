static int f1(int *p, int *q) { *p += 991; return *q * 2; }
static int f2(int *p, int *q) { *p += 537; return *q * 2; }
static int f3(int *p, int *q) { *p += 479; return *q * 2; }
static int f4(int *p, int *q) { *p += 619; return *q * 2; }
static int f5(int *p, int *q) { *p += 656; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 912u; }
    cells[7] = cells[4] + 860;
    cells[7] = cells[6] + 694;
    sum += f1(&cells[6], &cells[6]);
    sum += f2(&cells[7], &cells[0]);
    sum += f3(&cells[3], &cells[6]);
    arr[4 % 6] = 195; cells[6] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    sum += f4(&cells[7], &cells[2]);
    arr[2 % 6] = 845; cells[4] += arr[2 % 6];
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    cells[6] = cells[6] + 325;
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    arr[6 % 6] = 334; cells[2] += arr[6 % 6];
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 93u; }
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    sum += f5(&cells[1], &cells[2]);
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    arr[4 % 6] = 807; cells[5] += arr[4 % 6];
    cells[3] = cells[3] + 673;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
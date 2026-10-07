static int f1(int *p, int *q) { *p += 526; return *q * 2; }
static int f2(int *p, int *q) { *p += 882; return *q * 2; }
static int f3(int *p, int *q) { *p += 320; return *q * 2; }
static int f4(int *p, int *q) { *p += 978; return *q * 2; }
static int f5(int *p, int *q) { *p += 247; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    sum += f1(&cells[2], &cells[7]);
    sum += f2(&cells[2], &cells[0]);
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 795u; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    arr[0 % 6] = 878; cells[0] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    cells[3] = cells[3] + 288;
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    arr[0 % 6] = 281; cells[2] += arr[0 % 6];
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 567u; }
    sum += f3(&cells[3], &cells[6]);
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 71u; }
    cells[2] = cells[5] + 346;
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    sum += f4(&cells[1], &cells[4]);
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    sum += f5(&cells[7], &cells[2]);
    arr[0 % 6] = 893; cells[3] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
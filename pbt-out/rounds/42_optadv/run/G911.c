static int f1(int *p, int *q) { *p += 605; return *q * 2; }
static int f2(int *p, int *q) { *p += 828; return *q * 2; }
static int f3(int *p, int *q) { *p += 354; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[5] + 763;
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 134u; }
    sum += f1(&cells[4], &cells[6]);
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 792u; }
    arr[0 % 6] = 348; cells[7] += arr[0 % 6];
    sum += f2(&cells[4], &cells[5]);
    cells[4] = cells[1] + 664;
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 128u; }
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    arr[4 % 6] = 224; cells[1] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    arr[4 % 6] = 573; cells[3] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 274u; }
    arr[5 % 6] = 293; cells[7] += arr[5 % 6];
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 472u; }
    sum += f3(&cells[5], &cells[6]);
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    cells[0] = cells[7] + 5;
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
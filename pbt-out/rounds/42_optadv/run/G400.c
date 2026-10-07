static int f1(int *p, int *q) { *p += 38; return *q * 2; }
static int f2(int *p, int *q) { *p += 999; return *q * 2; }
static int f3(int *p, int *q) { *p += 295; return *q * 2; }
static int f4(int *p, int *q) { *p += 411; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    sum += f1(&cells[3], &cells[6]);
    cells[4] = cells[2] + 7;
    cells[1] = cells[0] + 880;
    sum += f2(&cells[1], &cells[7]);
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    cells[1] = cells[0] + 877;
    arr[2 % 6] = 442; cells[2] += arr[2 % 6];
    sum += f3(&cells[6], &cells[6]);
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    sum += f4(&cells[5], &cells[5]);
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    cells[4] = cells[4] + 10;
    cells[3] = cells[6] + 11;
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 648u; }
    cells[1] = cells[5] + 394;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
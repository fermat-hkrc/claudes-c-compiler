static int f1(int *p, int *q) { *p += 895; return *q * 2; }
static int f2(int *p, int *q) { *p += 347; return *q * 2; }
static int f3(int *p, int *q) { *p += 112; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    sum += f1(&cells[1], &cells[6]);
    sum += f2(&cells[3], &cells[5]);
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    arr[2 % 6] = 687; cells[2] += arr[2 % 6];
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 94u; }
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    sum += f3(&cells[3], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
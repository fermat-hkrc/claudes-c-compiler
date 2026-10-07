static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    arr[0 % 6] = 239; cells[4] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    arr[2 % 6] = 960; cells[4] += arr[2 % 6];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    arr[4 % 6] = 586; cells[3] += arr[4 % 6];
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 775u; }
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    arr[3 % 6] = 630; cells[3] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    cells[4] = cells[0] + 120;
    static int f1(int *p, int *q) { *p += 376; return *q * 2; }
    sum += f1(&cells[0], &cells[6]);
    static int f2(int *p, int *q) { *p += 340; return *q * 2; }
    sum += f2(&cells[5], &cells[1]);
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
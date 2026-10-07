static int f1(int *p, int *q) { *p += 882; return *q * 2; }
static int f2(int *p, int *q) { *p += 97; return *q * 2; }
static int f3(int *p, int *q) { *p += 936; return *q * 2; }
static int f4(int *p, int *q) { *p += 278; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[1]);
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    sum += f2(&cells[5], &cells[7]);
    cells[1] = cells[5] + 386;
    arr[6 % 6] = 917; cells[1] += arr[6 % 6];
    arr[2 % 6] = 894; cells[6] += arr[2 % 6];
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 923u; }
    sum += f3(&cells[5], &cells[0]);
    cells[6] = cells[3] + 954;
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    sum += f4(&cells[2], &cells[7]);
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 114u; }
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 149u; }
    arr[4 % 6] = 603; cells[0] += arr[4 % 6];
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    arr[0 % 6] = 855; cells[3] += arr[0 % 6];
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 840u; }
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 440u; }
    static int f1(int *p, int *q) { *p += 935; return *q * 2; }
    sum += f1(&cells[2], &cells[0]);
    static int f2(int *p, int *q) { *p += 333; return *q * 2; }
    sum += f2(&cells[3], &cells[5]);
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    static int f3(int *p, int *q) { *p += 261; return *q * 2; }
    sum += f3(&cells[0], &cells[6]);
    arr[0 % 6] = 657; cells[1] += arr[0 % 6];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 949u; }
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
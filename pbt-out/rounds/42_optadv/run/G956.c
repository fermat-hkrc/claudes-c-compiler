static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 666u; }
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    arr[5 % 6] = 976; cells[0] += arr[5 % 6];
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    static int f1(int *p, int *q) { *p += 809; return *q * 2; }
    sum += f1(&cells[6], &cells[5]);
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 356u; }
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    arr[7 % 6] = 722; cells[0] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
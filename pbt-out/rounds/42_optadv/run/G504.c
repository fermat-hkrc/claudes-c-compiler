static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 788u; }
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[2 % 6] = 385; cells[5] += arr[2 % 6];
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    static int f1(int *p, int *q) { *p += 150; return *q * 2; }
    sum += f1(&cells[0], &cells[7]);
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    static int f2(int *p, int *q) { *p += 984; return *q * 2; }
    sum += f2(&cells[3], &cells[2]);
    static int f3(int *p, int *q) { *p += 447; return *q * 2; }
    sum += f3(&cells[3], &cells[3]);
    arr[3 % 6] = 237; cells[2] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    cells[5] = cells[4] + 268;
    static int f4(int *p, int *q) { *p += 465; return *q * 2; }
    sum += f4(&cells[7], &cells[1]);
    cells[4] = cells[1] + 162;
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 863u; }
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 919u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
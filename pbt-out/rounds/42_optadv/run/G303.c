static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 927u; }
    arr[1 % 6] = 465; cells[6] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    arr[6 % 6] = 554; cells[7] += arr[6 % 6];
    cells[4] = cells[6] + 473;
    arr[7 % 6] = 965; cells[5] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 694u; }
    static int f1(int *p, int *q) { *p += 318; return *q * 2; }
    sum += f1(&cells[5], &cells[4]);
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 858u; }
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    static int f2(int *p, int *q) { *p += 734; return *q * 2; }
    sum += f2(&cells[2], &cells[4]);
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 900u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
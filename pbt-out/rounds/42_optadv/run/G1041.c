static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 199; cells[0] += arr[3 % 6];
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 193u; }
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 334u; }
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    static int f1(int *p, int *q) { *p += 536; return *q * 2; }
    sum += f1(&cells[6], &cells[0]);
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    cells[3] = cells[4] + 384;
    static int f2(int *p, int *q) { *p += 660; return *q * 2; }
    sum += f2(&cells[3], &cells[5]);
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    static int f3(int *p, int *q) { *p += 554; return *q * 2; }
    sum += f3(&cells[1], &cells[6]);
    cells[2] = cells[0] + 698;
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 836u; }
    static int f4(int *p, int *q) { *p += 365; return *q * 2; }
    sum += f4(&cells[0], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
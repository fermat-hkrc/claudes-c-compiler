static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 483; cells[7] += arr[0 % 6];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 776u; }
    static int f1(int *p, int *q) { *p += 517; return *q * 2; }
    sum += f1(&cells[6], &cells[1]);
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    static int f2(int *p, int *q) { *p += 374; return *q * 2; }
    sum += f2(&cells[2], &cells[0]);
    cells[1] = cells[1] + 131;
    static int f3(int *p, int *q) { *p += 28; return *q * 2; }
    sum += f3(&cells[4], &cells[0]);
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 404u; }
    static int f4(int *p, int *q) { *p += 542; return *q * 2; }
    sum += f4(&cells[5], &cells[0]);
    arr[7 % 6] = 630; cells[4] += arr[7 % 6];
    cells[1] = cells[6] + 771;
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 3u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 637u; }
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 747u; }
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 77u; }
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    static int f5(int *p, int *q) { *p += 813; return *q * 2; }
    sum += f5(&cells[7], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 54; cells[3] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 48; return *q * 2; }
    sum += f1(&cells[4], &cells[1]);
    static int f2(int *p, int *q) { *p += 501; return *q * 2; }
    sum += f2(&cells[1], &cells[2]);
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 352u; }
    cells[4] = cells[6] + 633;
    cells[4] = cells[2] + 617;
    static int f3(int *p, int *q) { *p += 776; return *q * 2; }
    sum += f3(&cells[0], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    static int f4(int *p, int *q) { *p += 305; return *q * 2; }
    sum += f4(&cells[7], &cells[6]);
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    static int f5(int *p, int *q) { *p += 473; return *q * 2; }
    sum += f5(&cells[5], &cells[6]);
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
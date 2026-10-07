static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 203; return *q * 2; }
    sum += f1(&cells[4], &cells[0]);
    static int f2(int *p, int *q) { *p += 998; return *q * 2; }
    sum += f2(&cells[7], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    static int f3(int *p, int *q) { *p += 564; return *q * 2; }
    sum += f3(&cells[4], &cells[2]);
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 352u; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    static int f4(int *p, int *q) { *p += 96; return *q * 2; }
    sum += f4(&cells[0], &cells[6]);
    cells[6] = cells[3] + 225;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    static int f5(int *p, int *q) { *p += 31; return *q * 2; }
    sum += f5(&cells[6], &cells[5]);
    arr[0 % 6] = 624; cells[4] += arr[0 % 6];
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 248u; }
    arr[0 % 6] = 400; cells[4] += arr[0 % 6];
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 744u; }
    static int f6(int *p, int *q) { *p += 660; return *q * 2; }
    sum += f6(&cells[5], &cells[2]);
    cells[6] = cells[5] + 54;
    static int f7(int *p, int *q) { *p += 618; return *q * 2; }
    sum += f7(&cells[7], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
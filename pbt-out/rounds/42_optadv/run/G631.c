static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[5] = cells[6] + 501;
    static int f1(int *p, int *q) { *p += 720; return *q * 2; }
    sum += f1(&cells[5], &cells[0]);
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 640u; }
    static int f2(int *p, int *q) { *p += 347; return *q * 2; }
    sum += f2(&cells[4], &cells[0]);
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f3(int *p, int *q) { *p += 399; return *q * 2; }
    sum += f3(&cells[5], &cells[2]);
    static int f4(int *p, int *q) { *p += 304; return *q * 2; }
    sum += f4(&cells[6], &cells[4]);
    cells[3] = cells[5] + 54;
    cells[0] = cells[2] + 225;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    cells[2] = cells[3] + 881;
    arr[6 % 6] = 634; cells[1] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    static int f5(int *p, int *q) { *p += 519; return *q * 2; }
    sum += f5(&cells[1], &cells[2]);
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    arr[5 % 6] = 314; cells[1] += arr[5 % 6];
    cells[5] = cells[3] + 73;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
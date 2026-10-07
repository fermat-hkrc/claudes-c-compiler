static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 262; cells[4] += arr[6 % 6];
    static int f1(int *p, int *q) { *p += 25; return *q * 2; }
    sum += f1(&cells[5], &cells[5]);
    cells[0] = cells[2] + 738;
    arr[7 % 6] = 729; cells[0] += arr[7 % 6];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 35u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f2(int *p, int *q) { *p += 672; return *q * 2; }
    sum += f2(&cells[1], &cells[3]);
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 796u; }
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    cells[0] = cells[3] + 60;
    cells[4] = cells[6] + 222;
    static int f3(int *p, int *q) { *p += 622; return *q * 2; }
    sum += f3(&cells[4], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[1] = cells[6] + 18;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[5] = cells[2] + 335;
    arr[2 % 6] = 530; cells[1] += arr[2 % 6];
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    cells[4] = cells[0] + 700;
    static int f4(int *p, int *q) { *p += 295; return *q * 2; }
    sum += f4(&cells[4], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
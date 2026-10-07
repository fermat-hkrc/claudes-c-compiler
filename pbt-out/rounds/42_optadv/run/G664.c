static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    static int f1(int *p, int *q) { *p += 742; return *q * 2; }
    sum += f1(&cells[5], &cells[6]);
    static int f2(int *p, int *q) { *p += 150; return *q * 2; }
    sum += f2(&cells[6], &cells[7]);
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 5u; }
    cells[7] = cells[5] + 972;
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    cells[0] = cells[7] + 200;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 640u; }
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    cells[5] = cells[6] + 387;
    cells[2] = cells[2] + 174;
    arr[0 % 6] = 616; cells[5] += arr[0 % 6];
    static int f3(int *p, int *q) { *p += 317; return *q * 2; }
    sum += f3(&cells[6], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f4(int *p, int *q) { *p += 366; return *q * 2; }
    sum += f4(&cells[7], &cells[7]);
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
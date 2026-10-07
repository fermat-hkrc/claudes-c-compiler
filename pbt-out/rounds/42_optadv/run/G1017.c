static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    static int f1(int *p, int *q) { *p += 40; return *q * 2; }
    sum += f1(&cells[7], &cells[2]);
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    static int f2(int *p, int *q) { *p += 775; return *q * 2; }
    sum += f2(&cells[4], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[2] = cells[0] + 938;
    cells[3] = cells[4] + 98;
    arr[4 % 6] = 683; cells[1] += arr[4 % 6];
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    static int f3(int *p, int *q) { *p += 803; return *q * 2; }
    sum += f3(&cells[0], &cells[6]);
    static int f4(int *p, int *q) { *p += 724; return *q * 2; }
    sum += f4(&cells[4], &cells[2]);
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 442u; }
    arr[3 % 6] = 285; cells[0] += arr[3 % 6];
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 344u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[5] + 799;
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    arr[0 % 6] = 872; cells[7] += arr[0 % 6];
    arr[2 % 6] = 557; cells[1] += arr[2 % 6];
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 956u; }
    static int f1(int *p, int *q) { *p += 90; return *q * 2; }
    sum += f1(&cells[3], &cells[5]);
    cells[2] = cells[3] + 884;
    static int f2(int *p, int *q) { *p += 439; return *q * 2; }
    sum += f2(&cells[5], &cells[3]);
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 214u; }
    static int f3(int *p, int *q) { *p += 735; return *q * 2; }
    sum += f3(&cells[0], &cells[6]);
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 709u; }
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 657u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    static int f4(int *p, int *q) { *p += 29; return *q * 2; }
    sum += f4(&cells[2], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[3] = cells[4] + 409;
    arr[3 % 6] = 214; cells[0] += arr[3 % 6];
    cells[5] = cells[6] + 262;
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 804u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
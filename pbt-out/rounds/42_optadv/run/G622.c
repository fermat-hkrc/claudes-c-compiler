static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 54; cells[4] += arr[6 % 6];
    cells[7] = cells[4] + 277;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 477u; }
    static int f1(int *p, int *q) { *p += 436; return *q * 2; }
    sum += f1(&cells[2], &cells[7]);
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    static int f2(int *p, int *q) { *p += 832; return *q * 2; }
    sum += f2(&cells[6], &cells[6]);
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    arr[4 % 6] = 756; cells[2] += arr[4 % 6];
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 296u; }
    cells[6] = cells[2] + 803;
    static int f3(int *p, int *q) { *p += 694; return *q * 2; }
    sum += f3(&cells[3], &cells[3]);
    arr[3 % 6] = 95; cells[7] += arr[3 % 6];
    arr[1 % 6] = 810; cells[4] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    arr[7 % 6] = 52; cells[1] += arr[7 % 6];
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 327u; }
    cells[1] = cells[7] + 827;
    cells[4] = cells[3] + 879;
    arr[2 % 6] = 856; cells[5] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
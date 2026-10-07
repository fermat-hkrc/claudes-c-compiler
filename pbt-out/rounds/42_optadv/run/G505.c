static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 113; return *q * 2; }
    sum += f1(&cells[7], &cells[3]);
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    static int f2(int *p, int *q) { *p += 949; return *q * 2; }
    sum += f2(&cells[2], &cells[2]);
    arr[0 % 6] = 14; cells[6] += arr[0 % 6];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    arr[2 % 6] = 603; cells[2] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[2] = cells[5] + 46;
    static int f3(int *p, int *q) { *p += 687; return *q * 2; }
    sum += f3(&cells[3], &cells[7]);
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 320u; }
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 42u; }
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 347u; }
    static int f4(int *p, int *q) { *p += 622; return *q * 2; }
    sum += f4(&cells[1], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[4 % 6] = 309; cells[4] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    cells[2] = cells[2] + 758;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
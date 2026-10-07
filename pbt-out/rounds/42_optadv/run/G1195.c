static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 410u; }
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    cells[0] = cells[3] + 57;
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 443u; }
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[6] = cells[7] + 712;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    static int f1(int *p, int *q) { *p += 341; return *q * 2; }
    sum += f1(&cells[7], &cells[7]);
    arr[6 % 6] = 770; cells[0] += arr[6 % 6];
    arr[4 % 6] = 192; cells[6] += arr[4 % 6];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[1] = cells[7] + 598;
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 247u; }
    static int f2(int *p, int *q) { *p += 485; return *q * 2; }
    sum += f2(&cells[2], &cells[0]);
    static int f3(int *p, int *q) { *p += 689; return *q * 2; }
    sum += f3(&cells[1], &cells[5]);
    static int f4(int *p, int *q) { *p += 196; return *q * 2; }
    sum += f4(&cells[6], &cells[2]);
    arr[4 % 6] = 855; cells[7] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
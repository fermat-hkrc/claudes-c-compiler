static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 528u; }
    static int f1(int *p, int *q) { *p += 911; return *q * 2; }
    sum += f1(&cells[2], &cells[2]);
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    cells[4] = cells[4] + 423;
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 202u; }
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 238u; }
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 978u; }
    static int f2(int *p, int *q) { *p += 303; return *q * 2; }
    sum += f2(&cells[3], &cells[0]);
    cells[0] = cells[1] + 393;
    static int f3(int *p, int *q) { *p += 19; return *q * 2; }
    sum += f3(&cells[6], &cells[6]);
    cells[4] = cells[3] + 158;
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 720u; }
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[4 % 6] = 132; cells[5] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 791u; }
    static int f4(int *p, int *q) { *p += 292; return *q * 2; }
    sum += f4(&cells[1], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
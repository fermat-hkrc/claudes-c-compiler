static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    static int f1(int *p, int *q) { *p += 450; return *q * 2; }
    sum += f1(&cells[3], &cells[6]);
    arr[0 % 6] = 293; cells[2] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f2(int *p, int *q) { *p += 139; return *q * 2; }
    sum += f2(&cells[7], &cells[2]);
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 217u; }
    arr[5 % 6] = 94; cells[3] += arr[5 % 6];
    cells[7] = cells[3] + 396;
    cells[4] = cells[2] + 337;
    cells[4] = cells[0] + 595;
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    cells[7] = cells[2] + 508;
    static int f3(int *p, int *q) { *p += 170; return *q * 2; }
    sum += f3(&cells[2], &cells[2]);
    cells[1] = cells[6] + 555;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f4(int *p, int *q) { *p += 389; return *q * 2; }
    sum += f4(&cells[4], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
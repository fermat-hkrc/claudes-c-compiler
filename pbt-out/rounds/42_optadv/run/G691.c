static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[3] + 246;
    static int f1(int *p, int *q) { *p += 8; return *q * 2; }
    sum += f1(&cells[3], &cells[0]);
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    cells[3] = cells[3] + 147;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[4] = cells[7] + 339;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 468u; }
    static int f2(int *p, int *q) { *p += 670; return *q * 2; }
    sum += f2(&cells[3], &cells[1]);
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    cells[6] = cells[4] + 939;
    static int f3(int *p, int *q) { *p += 390; return *q * 2; }
    sum += f3(&cells[4], &cells[0]);
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    cells[5] = cells[3] + 256;
    arr[6 % 6] = 593; cells[4] += arr[6 % 6];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 389u; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
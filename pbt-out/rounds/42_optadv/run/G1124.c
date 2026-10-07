static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    { int *p = &cells[7]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 646u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    static int f1(int *p, int *q) { *p += 876; return *q * 2; }
    sum += f1(&cells[7], &cells[2]);
    arr[0 % 6] = 797; cells[6] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 602u; }
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    static int f2(int *p, int *q) { *p += 811; return *q * 2; }
    sum += f2(&cells[7], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f3(int *p, int *q) { *p += 281; return *q * 2; }
    sum += f3(&cells[4], &cells[3]);
    sum += cells[5] > cells[3] ? cells[5] : -cells[3];
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 342u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f4(int *p, int *q) { *p += 935; return *q * 2; }
    sum += f4(&cells[2], &cells[5]);
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    cells[5] = cells[3] + 389;
    cells[1] = cells[1] + 416;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
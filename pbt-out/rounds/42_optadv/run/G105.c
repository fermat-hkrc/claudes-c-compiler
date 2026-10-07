static int f1(int *p, int *q) { *p += 990; return *q * 2; }
static int f2(int *p, int *q) { *p += 422; return *q * 2; }
static int f3(int *p, int *q) { *p += 348; return *q * 2; }
static int f4(int *p, int *q) { *p += 328; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    sum += f1(&cells[7], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 492u; }
    cells[6] = cells[6] + 661;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[7 % 6] = 130; cells[5] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[1] = cells[5] + 178;
    cells[4] = cells[3] + 636;
    sum += f2(&cells[1], &cells[4]);
    sum += f3(&cells[4], &cells[4]);
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    arr[4 % 6] = 935; cells[4] += arr[4 % 6];
    cells[0] = cells[2] + 868;
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 925u; }
    sum += f4(&cells[3], &cells[6]);
    cells[5] = cells[3] + 335;
    arr[2 % 6] = 368; cells[0] += arr[2 % 6];
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    cells[4] = cells[3] + 46;
    cells[2] = cells[4] + 808;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
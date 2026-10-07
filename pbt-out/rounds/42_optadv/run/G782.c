static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 877u; }
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    static int f1(int *p, int *q) { *p += 253; return *q * 2; }
    sum += f1(&cells[4], &cells[0]);
    cells[4] = cells[7] + 760;
    cells[4] = cells[3] + 611;
    cells[4] = cells[1] + 243;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    static int f2(int *p, int *q) { *p += 512; return *q * 2; }
    sum += f2(&cells[7], &cells[6]);
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 714u; }
    arr[7 % 6] = 414; cells[0] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 792u; }
    static int f3(int *p, int *q) { *p += 949; return *q * 2; }
    sum += f3(&cells[7], &cells[4]);
    arr[3 % 6] = 612; cells[0] += arr[3 % 6];
    cells[7] = cells[6] + 748;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
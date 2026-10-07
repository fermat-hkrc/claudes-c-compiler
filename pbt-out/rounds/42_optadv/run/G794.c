static int f1(int *p, int *q) { *p += 452; return *q * 2; }
static int f2(int *p, int *q) { *p += 275; return *q * 2; }
static int f3(int *p, int *q) { *p += 871; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += f1(&cells[5], &cells[5]);
    arr[0 % 6] = 818; cells[1] += arr[0 % 6];
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 691u; }
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    cells[7] = cells[3] + 580;
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 762u; }
    arr[5 % 6] = 37; cells[2] += arr[5 % 6];
    sum += f2(&cells[7], &cells[1]);
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    sum += f3(&cells[2], &cells[2]);
    cells[2] = cells[3] + 330;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    cells[6] = cells[7] + 902;
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[0 % 6] = 491; cells[1] += arr[0 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
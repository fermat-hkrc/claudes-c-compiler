static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 729; return *q * 2; }
    sum += f1(&cells[6], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[1] = cells[0] + 765;
    static int f2(int *p, int *q) { *p += 285; return *q * 2; }
    sum += f2(&cells[0], &cells[1]);
    cells[4] = cells[1] + 272;
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    cells[6] = cells[4] + 840;
    arr[4 % 6] = 850; cells[1] += arr[4 % 6];
    arr[1 % 6] = 561; cells[3] += arr[1 % 6];
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 587u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[5] = cells[0] + 835;
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 125; return *q * 2; }
    sum += f1(&cells[3], &cells[4]);
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    arr[4 % 6] = 798; cells[6] += arr[4 % 6];
    cells[3] = cells[2] + 739;
    cells[2] = cells[0] + 937;
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    static int f2(int *p, int *q) { *p += 880; return *q * 2; }
    sum += f2(&cells[5], &cells[7]);
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 27u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 129u; }
    arr[1 % 6] = 441; cells[5] += arr[1 % 6];
    static int f3(int *p, int *q) { *p += 185; return *q * 2; }
    sum += f3(&cells[0], &cells[5]);
    static int f4(int *p, int *q) { *p += 136; return *q * 2; }
    sum += f4(&cells[4], &cells[3]);
    cells[7] = cells[4] + 578;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
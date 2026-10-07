static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[5] = cells[7] + 444;
    arr[2 % 6] = 571; cells[3] += arr[2 % 6];
    arr[2 % 6] = 704; cells[4] += arr[2 % 6];
    static int f1(int *p, int *q) { *p += 419; return *q * 2; }
    sum += f1(&cells[2], &cells[0]);
    static int f2(int *p, int *q) { *p += 354; return *q * 2; }
    sum += f2(&cells[1], &cells[5]);
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    cells[0] = cells[0] + 401;
    static int f3(int *p, int *q) { *p += 136; return *q * 2; }
    sum += f3(&cells[7], &cells[0]);
    arr[0 % 6] = 317; cells[2] += arr[0 % 6];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 723u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 913u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
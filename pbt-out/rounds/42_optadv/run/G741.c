static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    static int f1(int *p, int *q) { *p += 179; return *q * 2; }
    sum += f1(&cells[2], &cells[2]);
    static int f2(int *p, int *q) { *p += 980; return *q * 2; }
    sum += f2(&cells[1], &cells[6]);
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    arr[1 % 6] = 549; cells[4] += arr[1 % 6];
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    arr[7 % 6] = 343; cells[7] += arr[7 % 6];
    static int f3(int *p, int *q) { *p += 762; return *q * 2; }
    sum += f3(&cells[0], &cells[7]);
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 1u; }
    arr[5 % 6] = 107; cells[2] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
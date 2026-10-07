static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 140; cells[4] += arr[2 % 6];
    static int f1(int *p, int *q) { *p += 925; return *q * 2; }
    sum += f1(&cells[5], &cells[7]);
    arr[0 % 6] = 514; cells[2] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    static int f2(int *p, int *q) { *p += 584; return *q * 2; }
    sum += f2(&cells[6], &cells[0]);
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 201u; }
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    cells[4] = cells[4] + 205;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    static int f3(int *p, int *q) { *p += 822; return *q * 2; }
    sum += f3(&cells[2], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
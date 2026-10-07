static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    arr[1 % 6] = 674; cells[0] += arr[1 % 6];
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    arr[6 % 6] = 686; cells[2] += arr[6 % 6];
    static int f1(int *p, int *q) { *p += 953; return *q * 2; }
    sum += f1(&cells[3], &cells[0]);
    static int f2(int *p, int *q) { *p += 601; return *q * 2; }
    sum += f2(&cells[3], &cells[2]);
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    cells[4] = cells[0] + 496;
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    static int f3(int *p, int *q) { *p += 817; return *q * 2; }
    sum += f3(&cells[7], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 488u; }
    static int f4(int *p, int *q) { *p += 991; return *q * 2; }
    sum += f4(&cells[7], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
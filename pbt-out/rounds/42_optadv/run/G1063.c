static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[2 % 6] = 625; cells[1] += arr[2 % 6];
    static int f1(int *p, int *q) { *p += 906; return *q * 2; }
    sum += f1(&cells[0], &cells[4]);
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 415u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    static int f2(int *p, int *q) { *p += 288; return *q * 2; }
    sum += f2(&cells[3], &cells[0]);
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    static int f3(int *p, int *q) { *p += 739; return *q * 2; }
    sum += f3(&cells[1], &cells[0]);
    static int f4(int *p, int *q) { *p += 2; return *q * 2; }
    sum += f4(&cells[1], &cells[3]);
    cells[4] = cells[1] + 313;
    cells[2] = cells[5] + 971;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f5(int *p, int *q) { *p += 904; return *q * 2; }
    sum += f5(&cells[2], &cells[2]);
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
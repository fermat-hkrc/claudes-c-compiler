static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 762u; }
    static int f1(int *p, int *q) { *p += 291; return *q * 2; }
    sum += f1(&cells[3], &cells[5]);
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    static int f2(int *p, int *q) { *p += 682; return *q * 2; }
    sum += f2(&cells[7], &cells[4]);
    cells[0] = cells[6] + 53;
    static int f3(int *p, int *q) { *p += 89; return *q * 2; }
    sum += f3(&cells[4], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    static int f4(int *p, int *q) { *p += 770; return *q * 2; }
    sum += f4(&cells[2], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
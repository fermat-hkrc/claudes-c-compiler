static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 389; return *q * 2; }
    sum += f1(&cells[4], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[1] = cells[4] + 762;
    static int f2(int *p, int *q) { *p += 944; return *q * 2; }
    sum += f2(&cells[3], &cells[4]);
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 941u; }
    static int f3(int *p, int *q) { *p += 979; return *q * 2; }
    sum += f3(&cells[6], &cells[0]);
    cells[3] = cells[4] + 933;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[5 % 6] = 605; cells[7] += arr[5 % 6];
    cells[2] = cells[5] + 678;
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
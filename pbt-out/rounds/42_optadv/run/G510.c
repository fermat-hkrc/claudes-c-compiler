static int f1(int *p, int *q) { *p += 986; return *q * 2; }
static int f2(int *p, int *q) { *p += 709; return *q * 2; }
static int f3(int *p, int *q) { *p += 671; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 354u; }
    sum += f1(&cells[3], &cells[6]);
    cells[7] = cells[1] + 327;
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    cells[6] = cells[3] + 39;
    sum += f2(&cells[1], &cells[6]);
    cells[0] = cells[6] + 66;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 961u; }
    sum += f3(&cells[4], &cells[4]);
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 529u; }
    arr[1 % 6] = 160; cells[6] += arr[1 % 6];
    { int *p = &cells[3]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 636u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
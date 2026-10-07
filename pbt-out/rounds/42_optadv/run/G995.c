static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 386; return *q * 2; }
    sum += f1(&cells[0], &cells[7]);
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 802u; }
    arr[5 % 6] = 852; cells[4] += arr[5 % 6];
    static int f2(int *p, int *q) { *p += 491; return *q * 2; }
    sum += f2(&cells[1], &cells[7]);
    static int f3(int *p, int *q) { *p += 317; return *q * 2; }
    sum += f3(&cells[1], &cells[0]);
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 871u; }
    cells[2] = cells[4] + 980;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 380u; }
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 901u; }
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    cells[4] = cells[3] + 487;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
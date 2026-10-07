static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[5] + 390;
    cells[4] = cells[2] + 520;
    cells[0] = cells[5] + 485;
    cells[4] = cells[1] + 295;
    cells[2] = cells[6] + 592;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[0 % 6] = 112; cells[2] += arr[0 % 6];
    arr[6 % 6] = 464; cells[4] += arr[6 % 6];
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 788u; }
    cells[5] = cells[5] + 978;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    cells[1] = cells[7] + 858;
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 568u; }
    static int f1(int *p, int *q) { *p += 925; return *q * 2; }
    sum += f1(&cells[3], &cells[4]);
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 382u; }
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 209u; }
    cells[4] = cells[2] + 768;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
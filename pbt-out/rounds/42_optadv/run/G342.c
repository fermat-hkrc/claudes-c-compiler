static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 521u; }
    arr[6 % 6] = 239; cells[7] += arr[6 % 6];
    cells[2] = cells[6] + 607;
    cells[3] = cells[3] + 629;
    arr[0 % 6] = 864; cells[5] += arr[0 % 6];
    cells[4] = cells[5] + 448;
    cells[7] = cells[4] + 168;
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 366u; }
    static int f1(int *p, int *q) { *p += 530; return *q * 2; }
    sum += f1(&cells[4], &cells[6]);
    arr[3 % 6] = 318; cells[3] += arr[3 % 6];
    arr[3 % 6] = 82; cells[1] += arr[3 % 6];
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
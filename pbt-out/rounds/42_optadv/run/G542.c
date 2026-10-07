static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    arr[0 % 6] = 43; cells[5] += arr[0 % 6];
    arr[6 % 6] = 609; cells[7] += arr[6 % 6];
    arr[6 % 6] = 653; cells[0] += arr[6 % 6];
    arr[3 % 6] = 992; cells[3] += arr[3 % 6];
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 325u; }
    static int f1(int *p, int *q) { *p += 89; return *q * 2; }
    sum += f1(&cells[0], &cells[5]);
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
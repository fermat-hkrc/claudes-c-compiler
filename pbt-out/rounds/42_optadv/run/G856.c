static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 311; return *q * 2; }
    sum += f1(&cells[5], &cells[4]);
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    arr[7 % 6] = 392; cells[7] += arr[7 % 6];
    cells[1] = cells[6] + 419;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 1u; }
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    arr[7 % 6] = 637; cells[4] += arr[7 % 6];
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 237u; }
    arr[2 % 6] = 798; cells[1] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    arr[3 % 6] = 57; cells[1] += arr[3 % 6];
    arr[3 % 6] = 503; cells[7] += arr[3 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
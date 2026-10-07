static int f1(int *p, int *q) { *p += 54; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 765; cells[3] += arr[4 % 6];
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 204u; }
    arr[0 % 6] = 577; cells[3] += arr[0 % 6];
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 71u; }
    arr[7 % 6] = 38; cells[4] += arr[7 % 6];
    cells[5] = cells[2] + 277;
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    cells[7] = cells[0] + 865;
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    arr[4 % 6] = 363; cells[3] += arr[4 % 6];
    sum += f1(&cells[0], &cells[7]);
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
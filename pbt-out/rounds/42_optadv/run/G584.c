static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[1] + 304;
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 27u; }
    arr[6 % 6] = 499; cells[4] += arr[6 % 6];
    cells[7] = cells[1] + 137;
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 98u; }
    cells[4] = cells[7] + 314;
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[3 % 6] = 930; cells[7] += arr[3 % 6];
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    arr[0 % 6] = 653; cells[5] += arr[0 % 6];
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    cells[0] = cells[2] + 511;
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 100u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
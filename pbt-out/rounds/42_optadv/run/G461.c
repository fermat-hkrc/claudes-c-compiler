static int f1(int *p, int *q) { *p += 229; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    cells[6] = cells[3] + 769;
    arr[6 % 6] = 200; cells[3] += arr[6 % 6];
    cells[2] = cells[3] + 733;
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 369u; }
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    arr[5 % 6] = 638; cells[5] += arr[5 % 6];
    arr[5 % 6] = 151; cells[0] += arr[5 % 6];
    arr[7 % 6] = 116; cells[3] += arr[7 % 6];
    cells[6] = cells[6] + 405;
    cells[0] = cells[7] + 648;
    sum += f1(&cells[4], &cells[3]);
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 795u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
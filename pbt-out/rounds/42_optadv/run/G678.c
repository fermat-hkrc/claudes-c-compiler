static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[1] + 618;
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 253u; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    arr[6 % 6] = 606; cells[0] += arr[6 % 6];
    arr[0 % 6] = 118; cells[7] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    static int f1(int *p, int *q) { *p += 842; return *q * 2; }
    sum += f1(&cells[1], &cells[6]);
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    cells[4] = cells[2] + 307;
    cells[3] = cells[3] + 543;
    static int f2(int *p, int *q) { *p += 797; return *q * 2; }
    sum += f2(&cells[0], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    cells[3] = cells[2] + 873;
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 904u; }
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 729u; }
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 641u; }
    static int f1(int *p, int *q) { *p += 278; return *q * 2; }
    sum += f1(&cells[5], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[3 % 6] = 931; cells[7] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 217u; }
    static int f2(int *p, int *q) { *p += 286; return *q * 2; }
    sum += f2(&cells[2], &cells[4]);
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    cells[7] = cells[3] + 155;
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 91u; }
    cells[0] = cells[1] + 816;
    cells[4] = cells[5] + 985;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
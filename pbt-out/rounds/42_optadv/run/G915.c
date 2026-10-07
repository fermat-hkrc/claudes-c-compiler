static int f1(int *p, int *q) { *p += 25; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 253u; }
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 153u; }
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 636u; }
    cells[0] = cells[4] + 907;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[7]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 12u; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    arr[4 % 6] = 651; cells[6] += arr[4 % 6];
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    sum += f1(&cells[5], &cells[4]);
    arr[6 % 6] = 50; cells[3] += arr[6 % 6];
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    cells[5] = cells[2] + 732;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
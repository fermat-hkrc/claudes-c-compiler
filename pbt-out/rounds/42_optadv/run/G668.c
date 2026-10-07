static int f1(int *p, int *q) { *p += 557; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[5]);
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[7] = cells[0] + 25;
    cells[0] = cells[4] + 772;
    arr[0 % 6] = 357; cells[2] += arr[0 % 6];
    arr[7 % 6] = 100; cells[7] += arr[7 % 6];
    arr[0 % 6] = 957; cells[3] += arr[0 % 6];
    cells[6] = cells[7] + 810;
    cells[3] = cells[7] + 453;
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 216u; }
    cells[5] = cells[7] + 821;
    arr[4 % 6] = 795; cells[0] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
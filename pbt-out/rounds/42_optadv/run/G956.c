static int f1(int *p, int *q) { *p += 922; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[4 % 6] = 107; cells[5] += arr[4 % 6];
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 493u; }
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    sum += f1(&cells[0], &cells[5]);
    arr[7 % 6] = 40; cells[6] += arr[7 % 6];
    cells[2] = cells[7] + 404;
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    cells[5] = cells[2] + 833;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
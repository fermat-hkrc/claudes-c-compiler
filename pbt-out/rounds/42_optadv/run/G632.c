static int f1(int *p, int *q) { *p += 667; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 447u; }
    { struct { int v; int *np; } box = { cells[6], &cells[1] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 499u; }
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    cells[3] = cells[7] + 623;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f1(&cells[1], &cells[2]);
    arr[7 % 6] = 444; cells[2] += arr[7 % 6];
    cells[5] = cells[7] + 372;
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
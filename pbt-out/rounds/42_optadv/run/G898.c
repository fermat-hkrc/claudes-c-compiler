static int f1(int *p, int *q) { *p += 460; return *q * 2; }
static int f2(int *p, int *q) { *p += 676; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    sum += f1(&cells[3], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f2(&cells[2], &cells[7]);
    arr[6 % 6] = 701; cells[3] += arr[6 % 6];
    { int *p = &cells[7]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 520u; }
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    cells[5] = cells[1] + 779;
    arr[0 % 6] = 953; cells[7] += arr[0 % 6];
    cells[7] = cells[0] + 198;
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
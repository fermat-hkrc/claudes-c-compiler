static int f1(int *p, int *q) { *p += 295; return *q * 2; }
static int f2(int *p, int *q) { *p += 680; return *q * 2; }
static int f3(int *p, int *q) { *p += 602; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[2], &cells[4]);
    arr[7 % 6] = 747; cells[0] += arr[7 % 6];
    sum += f2(&cells[3], &cells[3]);
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 561u; }
    sum += f3(&cells[6], &cells[5]);
    arr[2 % 6] = 722; cells[4] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[4] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    arr[4 % 6] = 585; cells[4] += arr[4 % 6];
    arr[2 % 6] = 781; cells[3] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
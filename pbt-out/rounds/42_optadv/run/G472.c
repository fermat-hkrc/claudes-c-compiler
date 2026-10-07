static int f1(int *p, int *q) { *p += 897; return *q * 2; }
static int f2(int *p, int *q) { *p += 120; return *q * 2; }
static int f3(int *p, int *q) { *p += 993; return *q * 2; }
static int f4(int *p, int *q) { *p += 675; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    sum += f1(&cells[3], &cells[0]);
    cells[1] = cells[6] + 33;
    sum += f2(&cells[7], &cells[3]);
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[4 % 6] = 494; cells[3] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    arr[1 % 6] = 907; cells[6] += arr[1 % 6];
    sum += f3(&cells[2], &cells[1]);
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 629u; }
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    sum += f4(&cells[4], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
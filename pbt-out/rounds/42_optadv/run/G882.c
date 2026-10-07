static int f1(int *p, int *q) { *p += 493; return *q * 2; }
static int f2(int *p, int *q) { *p += 663; return *q * 2; }
static int f3(int *p, int *q) { *p += 991; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    cells[2] = cells[2] + 223;
    arr[4 % 6] = 565; cells[3] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[6]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 134u; }
    sum += f1(&cells[5], &cells[0]);
    arr[5 % 6] = 872; cells[7] += arr[5 % 6];
    cells[1] = cells[6] + 852;
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    sum += f2(&cells[0], &cells[2]);
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    arr[2 % 6] = 739; cells[7] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f3(&cells[4], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
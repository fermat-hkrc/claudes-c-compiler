static int f1(int *p, int *q) { *p += 28; return *q * 2; }
static int f2(int *p, int *q) { *p += 798; return *q * 2; }
static int f3(int *p, int *q) { *p += 717; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    arr[2 % 6] = 865; cells[7] += arr[2 % 6];
    cells[7] = cells[6] + 878;
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    sum += f1(&cells[7], &cells[1]);
    sum += f2(&cells[3], &cells[6]);
    cells[7] = cells[2] + 428;
    sum += f3(&cells[6], &cells[7]);
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[1]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 196u; }
    cells[0] = cells[5] + 945;
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    cells[1] = cells[4] + 2;
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
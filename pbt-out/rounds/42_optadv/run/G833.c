static int f1(int *p, int *q) { *p += 797; return *q * 2; }
static int f2(int *p, int *q) { *p += 60; return *q * 2; }
static int f3(int *p, int *q) { *p += 704; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    arr[1 % 6] = 112; cells[2] += arr[1 % 6];
    cells[7] = cells[1] + 330;
    arr[4 % 6] = 681; cells[3] += arr[4 % 6];
    cells[1] = cells[3] + 933;
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    cells[5] = cells[1] + 839;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 736u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f1(&cells[1], &cells[5]);
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += f2(&cells[6], &cells[1]);
    cells[7] = cells[7] + 464;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    sum += f3(&cells[7], &cells[2]);
    arr[3 % 6] = 630; cells[0] += arr[3 % 6];
    arr[5 % 6] = 307; cells[0] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[5] + 920;
    cells[2] = cells[7] + 584;
    cells[5] = cells[7] + 914;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[4 % 6] = 692; cells[2] += arr[4 % 6];
    arr[0 % 6] = 890; cells[7] += arr[0 % 6];
    cells[6] = cells[3] + 772;
    cells[7] = cells[6] + 711;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[6 % 6] = 654; cells[7] += arr[6 % 6];
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 651u; }
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    arr[7 % 6] = 676; cells[0] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[1 % 6] = 272; cells[2] += arr[1 % 6];
    static int f1(int *p, int *q) { *p += 296; return *q * 2; }
    sum += f1(&cells[4], &cells[6]);
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 306u; }
    static int f2(int *p, int *q) { *p += 615; return *q * 2; }
    sum += f2(&cells[4], &cells[6]);
    { struct { int v; int *np; } box = { cells[4], &cells[4] }; box.v += *box.np; cells[4] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
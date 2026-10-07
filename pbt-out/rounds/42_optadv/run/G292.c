static int f1(int *p, int *q) { *p += 952; return *q * 2; }
static int f2(int *p, int *q) { *p += 295; return *q * 2; }
static int f3(int *p, int *q) { *p += 533; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[6] = cells[4] + 81;
    sum += f1(&cells[3], &cells[7]);
    sum += f2(&cells[1], &cells[3]);
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    sum += f3(&cells[3], &cells[5]);
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 591u; }
    arr[3 % 6] = 175; cells[6] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[4 % 6] = 159; cells[1] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    cells[0] = cells[2] + 877;
    arr[6 % 6] = 245; cells[5] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    arr[6 % 6] = 624; cells[2] += arr[6 % 6];
    arr[6 % 6] = 399; cells[2] += arr[6 % 6];
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
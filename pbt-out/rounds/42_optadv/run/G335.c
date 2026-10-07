static int f1(int *p, int *q) { *p += 319; return *q * 2; }
static int f2(int *p, int *q) { *p += 629; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 342; cells[2] += arr[4 % 6];
    arr[5 % 6] = 336; cells[7] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f1(&cells[5], &cells[2]);
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    arr[3 % 6] = 889; cells[4] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    arr[1 % 6] = 791; cells[0] += arr[1 % 6];
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    sum += f2(&cells[2], &cells[5]);
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    cells[4] = cells[1] + 234;
    arr[1 % 6] = 805; cells[6] += arr[1 % 6];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 434u; }
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 733u; }
    arr[0 % 6] = 955; cells[4] += arr[0 % 6];
    cells[7] = cells[5] + 39;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
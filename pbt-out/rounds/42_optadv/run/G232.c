static int f1(int *p, int *q) { *p += 785; return *q * 2; }
static int f2(int *p, int *q) { *p += 754; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 862; cells[0] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    arr[4 % 6] = 460; cells[7] += arr[4 % 6];
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    { struct { int v; int *np; } box = { cells[3], &cells[0] }; box.v += *box.np; cells[3] = box.v; }
    cells[5] = cells[5] + 780;
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    cells[4] = cells[4] + 380;
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    cells[2] = cells[1] + 293;
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    sum += f1(&cells[2], &cells[2]);
    cells[2] = cells[0] + 873;
    arr[5 % 6] = 328; cells[2] += arr[5 % 6];
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 314u; }
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[6] = cells[2] + 967;
    sum += f2(&cells[7], &cells[0]);
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
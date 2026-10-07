static int f1(int *p, int *q) { *p += 321; return *q * 2; }
static int f2(int *p, int *q) { *p += 789; return *q * 2; }
static int f3(int *p, int *q) { *p += 277; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    cells[7] = cells[6] + 490;
    sum += f1(&cells[6], &cells[4]);
    sum += f2(&cells[0], &cells[4]);
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    arr[3 % 6] = 214; cells[3] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 145u; }
    arr[5 % 6] = 235; cells[3] += arr[5 % 6];
    arr[2 % 6] = 626; cells[2] += arr[2 % 6];
    sum += f3(&cells[1], &cells[7]);
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 424u; }
    cells[2] = cells[4] + 726;
    arr[5 % 6] = 336; cells[0] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
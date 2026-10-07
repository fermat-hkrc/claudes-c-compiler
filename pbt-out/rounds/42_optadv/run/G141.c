static int f1(int *p, int *q) { *p += 983; return *q * 2; }
static int f2(int *p, int *q) { *p += 470; return *q * 2; }
static int f3(int *p, int *q) { *p += 177; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 480u; }
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 71u; }
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[1 % 6] = 156; cells[7] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    cells[6] = cells[2] + 587;
    arr[6 % 6] = 120; cells[4] += arr[6 % 6];
    cells[1] = cells[6] + 50;
    cells[4] = cells[1] + 275;
    sum += f1(&cells[0], &cells[3]);
    cells[6] = cells[2] + 694;
    sum += f2(&cells[0], &cells[4]);
    arr[3 % 6] = 276; cells[4] += arr[3 % 6];
    arr[0 % 6] = 490; cells[7] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 619u; }
    sum += f3(&cells[0], &cells[7]);
    { int *p = &cells[4]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 399u; }
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    arr[7 % 6] = 8; cells[2] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
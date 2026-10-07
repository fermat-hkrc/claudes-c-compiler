static int f1(int *p, int *q) { *p += 18; return *q * 2; }
static int f2(int *p, int *q) { *p += 14; return *q * 2; }
static int f3(int *p, int *q) { *p += 472; return *q * 2; }
static int f4(int *p, int *q) { *p += 605; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[2] = cells[3] + 192;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    arr[2 % 6] = 131; cells[0] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    sum += f1(&cells[6], &cells[6]);
    arr[7 % 6] = 73; cells[2] += arr[7 % 6];
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 177u; }
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    sum += f2(&cells[2], &cells[0]);
    sum += f3(&cells[4], &cells[5]);
    sum += f4(&cells[4], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 346u; }
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 334u; }
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
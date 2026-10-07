static int f1(int *p, int *q) { *p += 735; return *q * 2; }
static int f2(int *p, int *q) { *p += 572; return *q * 2; }
static int f3(int *p, int *q) { *p += 928; return *q * 2; }
static int f4(int *p, int *q) { *p += 914; return *q * 2; }
static int f5(int *p, int *q) { *p += 400; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[6 % 6] = 765; cells[6] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[3] = cells[3] + 420;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f1(&cells[6], &cells[2]);
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 375u; }
    cells[6] = cells[4] + 323;
    cells[6] = cells[0] + 777;
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 957u; }
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    sum += f2(&cells[4], &cells[6]);
    sum += f3(&cells[7], &cells[0]);
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    sum += f4(&cells[7], &cells[5]);
    sum += f5(&cells[1], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
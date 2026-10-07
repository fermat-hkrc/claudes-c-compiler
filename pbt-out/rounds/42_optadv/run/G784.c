static int f1(int *p, int *q) { *p += 436; return *q * 2; }
static int f2(int *p, int *q) { *p += 12; return *q * 2; }
static int f3(int *p, int *q) { *p += 978; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[1]);
    cells[4] = cells[3] + 37;
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f2(&cells[5], &cells[2]);
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    sum += f3(&cells[0], &cells[6]);
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    cells[2] = cells[7] + 359;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    cells[6] = cells[5] + 157;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
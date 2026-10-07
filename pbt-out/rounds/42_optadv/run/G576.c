static int f1(int *p, int *q) { *p += 930; return *q * 2; }
static int f2(int *p, int *q) { *p += 81; return *q * 2; }
static int f3(int *p, int *q) { *p += 59; return *q * 2; }
static int f4(int *p, int *q) { *p += 502; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    cells[3] = cells[2] + 893;
    sum += f1(&cells[4], &cells[5]);
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    sum += f2(&cells[2], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    sum += f3(&cells[6], &cells[2]);
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    cells[2] = cells[5] + 748;
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    sum += f4(&cells[5], &cells[3]);
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
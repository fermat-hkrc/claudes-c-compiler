static int f1(int *p, int *q) { *p += 990; return *q * 2; }
static int f2(int *p, int *q) { *p += 854; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    cells[2] = cells[4] + 875;
    sum += f1(&cells[1], &cells[4]);
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    cells[1] = cells[5] + 641;
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    arr[3 % 6] = 326; cells[7] += arr[3 % 6];
    sum += f2(&cells[1], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
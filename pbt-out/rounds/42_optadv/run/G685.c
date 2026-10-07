static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 118; return *q * 2; }
    sum += f1(&cells[3], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    static int f2(int *p, int *q) { *p += 871; return *q * 2; }
    sum += f2(&cells[5], &cells[2]);
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    static int f3(int *p, int *q) { *p += 894; return *q * 2; }
    sum += f3(&cells[2], &cells[1]);
    arr[1 % 6] = 321; cells[1] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[0 % 6] = 500; cells[5] += arr[0 % 6];
    cells[3] = cells[5] + 349;
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
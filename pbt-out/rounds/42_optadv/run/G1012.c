static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 858; cells[3] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    static int f1(int *p, int *q) { *p += 964; return *q * 2; }
    sum += f1(&cells[3], &cells[0]);
    static int f2(int *p, int *q) { *p += 449; return *q * 2; }
    sum += f2(&cells[4], &cells[4]);
    cells[7] = cells[5] + 477;
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    static int f3(int *p, int *q) { *p += 303; return *q * 2; }
    sum += f3(&cells[1], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[0 % 6] = 533; cells[1] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    cells[6] = cells[7] + 952;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
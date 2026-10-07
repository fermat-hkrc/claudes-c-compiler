static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    static int f1(int *p, int *q) { *p += 955; return *q * 2; }
    sum += f1(&cells[6], &cells[6]);
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    static int f2(int *p, int *q) { *p += 232; return *q * 2; }
    sum += f2(&cells[1], &cells[0]);
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    static int f3(int *p, int *q) { *p += 512; return *q * 2; }
    sum += f3(&cells[6], &cells[1]);
    cells[1] = cells[7] + 713;
    arr[6 % 6] = 815; cells[0] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    arr[5 % 6] = 474; cells[0] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
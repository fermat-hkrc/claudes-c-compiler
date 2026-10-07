static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 633; return *q * 2; }
    sum += f1(&cells[0], &cells[2]);
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    cells[2] = cells[6] + 33;
    static int f2(int *p, int *q) { *p += 813; return *q * 2; }
    sum += f2(&cells[0], &cells[1]);
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    arr[1 % 6] = 902; cells[4] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[7] = cells[4] + 52;
    arr[0 % 6] = 676; cells[2] += arr[0 % 6];
    arr[3 % 6] = 764; cells[2] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    cells[6] = cells[4] + 467;
    static int f3(int *p, int *q) { *p += 297; return *q * 2; }
    sum += f3(&cells[7], &cells[7]);
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[4 % 6] = 299; cells[0] += arr[4 % 6];
    arr[5 % 6] = 166; cells[5] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
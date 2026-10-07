static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    cells[2] = cells[1] + 901;
    static int f1(int *p, int *q) { *p += 547; return *q * 2; }
    sum += f1(&cells[3], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[2 % 6] = 465; cells[7] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    static int f2(int *p, int *q) { *p += 444; return *q * 2; }
    sum += f2(&cells[5], &cells[5]);
    arr[6 % 6] = 662; cells[4] += arr[6 % 6];
    arr[0 % 6] = 291; cells[6] += arr[0 % 6];
    arr[6 % 6] = 51; cells[2] += arr[6 % 6];
    arr[4 % 6] = 906; cells[5] += arr[4 % 6];
    arr[7 % 6] = 846; cells[1] += arr[7 % 6];
    arr[2 % 6] = 148; cells[7] += arr[2 % 6];
    arr[1 % 6] = 860; cells[6] += arr[1 % 6];
    arr[1 % 6] = 298; cells[7] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
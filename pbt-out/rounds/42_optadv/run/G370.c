static int f1(int *p, int *q) { *p += 419; return *q * 2; }
static int f2(int *p, int *q) { *p += 590; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 659; cells[5] += arr[7 % 6];
    arr[5 % 6] = 170; cells[3] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[3 % 6] = 206; cells[5] += arr[3 % 6];
    arr[4 % 6] = 732; cells[7] += arr[4 % 6];
    sum += f1(&cells[0], &cells[4]);
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    cells[0] = cells[0] + 510;
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    arr[1 % 6] = 92; cells[4] += arr[1 % 6];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    sum += f2(&cells[3], &cells[1]);
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
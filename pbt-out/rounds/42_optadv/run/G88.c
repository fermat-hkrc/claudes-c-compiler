static int f1(int *p, int *q) { *p += 327; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    sum += f1(&cells[1], &cells[4]);
    cells[3] = cells[2] + 176;
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    arr[1 % 6] = 710; cells[2] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
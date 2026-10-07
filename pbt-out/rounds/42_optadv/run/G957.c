static int f1(int *p, int *q) { *p += 625; return *q * 2; }
static int f2(int *p, int *q) { *p += 893; return *q * 2; }
static int f3(int *p, int *q) { *p += 517; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[4 % 6] = 696; cells[5] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f1(&cells[2], &cells[1]);
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += f2(&cells[4], &cells[1]);
    sum += f3(&cells[4], &cells[3]);
    arr[0 % 6] = 219; cells[1] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
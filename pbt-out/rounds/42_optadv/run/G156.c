static int f1(int *p, int *q) { *p += 696; return *q * 2; }
static int f2(int *p, int *q) { *p += 649; return *q * 2; }
static int f3(int *p, int *q) { *p += 449; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f1(&cells[0], &cells[0]);
    arr[3 % 6] = 591; cells[7] += arr[3 % 6];
    arr[2 % 6] = 649; cells[2] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    cells[0] = cells[6] + 860;
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    sum += f2(&cells[1], &cells[0]);
    arr[6 % 6] = 227; cells[5] += arr[6 % 6];
    sum += f3(&cells[2], &cells[4]);
    cells[4] = cells[3] + 644;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 745; return *q * 2; }
static int f2(int *p, int *q) { *p += 133; return *q * 2; }
static int f3(int *p, int *q) { *p += 259; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[4], &cells[7]);
    cells[5] = cells[7] + 879;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[3 % 6] = 421; cells[2] += arr[3 % 6];
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    arr[6 % 6] = 687; cells[1] += arr[6 % 6];
    arr[3 % 6] = 879; cells[7] += arr[3 % 6];
    sum += f2(&cells[7], &cells[1]);
    sum += f3(&cells[0], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
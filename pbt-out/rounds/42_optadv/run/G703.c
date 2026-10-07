static int f1(int *p, int *q) { *p += 510; return *q * 2; }
static int f2(int *p, int *q) { *p += 796; return *q * 2; }
static int f3(int *p, int *q) { *p += 547; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[1] + 979;
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    sum += f1(&cells[3], &cells[1]);
    sum += f2(&cells[7], &cells[6]);
    sum += f3(&cells[3], &cells[5]);
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    cells[1] = cells[5] + 846;
    arr[1 % 6] = 912; cells[5] += arr[1 % 6];
    arr[2 % 6] = 969; cells[3] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    arr[6 % 6] = 874; cells[0] += arr[6 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 279; return *q * 2; }
static int f2(int *p, int *q) { *p += 488; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[6]);
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    arr[7 % 6] = 249; cells[1] += arr[7 % 6];
    cells[5] = cells[3] + 232;
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    arr[0 % 6] = 122; cells[5] += arr[0 % 6];
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    arr[2 % 6] = 846; cells[1] += arr[2 % 6];
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    cells[0] = cells[5] + 43;
    sum += f2(&cells[2], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 398; cells[5] += arr[1 % 6];
    arr[6 % 6] = 911; cells[2] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    arr[5 % 6] = 841; cells[5] += arr[5 % 6];
    static int f1(int *p, int *q) { *p += 186; return *q * 2; }
    sum += f1(&cells[2], &cells[0]);
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    static int f2(int *p, int *q) { *p += 386; return *q * 2; }
    sum += f2(&cells[0], &cells[6]);
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    static int f3(int *p, int *q) { *p += 472; return *q * 2; }
    sum += f3(&cells[2], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
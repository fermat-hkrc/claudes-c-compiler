static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    static int f1(int *p, int *q) { *p += 344; return *q * 2; }
    sum += f1(&cells[1], &cells[1]);
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    static int f2(int *p, int *q) { *p += 628; return *q * 2; }
    sum += f2(&cells[3], &cells[3]);
    arr[5 % 6] = 481; cells[7] += arr[5 % 6];
    static int f3(int *p, int *q) { *p += 281; return *q * 2; }
    sum += f3(&cells[3], &cells[6]);
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
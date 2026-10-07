static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 778; return *q * 2; }
    sum += f1(&cells[6], &cells[4]);
    arr[3 % 6] = 250; cells[2] += arr[3 % 6];
    arr[7 % 6] = 573; cells[5] += arr[7 % 6];
    static int f2(int *p, int *q) { *p += 874; return *q * 2; }
    sum += f2(&cells[7], &cells[1]);
    static int f3(int *p, int *q) { *p += 7; return *q * 2; }
    sum += f3(&cells[6], &cells[1]);
    static int f4(int *p, int *q) { *p += 883; return *q * 2; }
    sum += f4(&cells[2], &cells[4]);
    cells[3] = cells[5] + 933;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    arr[3 % 6] = 493; cells[1] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
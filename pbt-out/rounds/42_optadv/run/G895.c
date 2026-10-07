static int f1(int *p, int *q) { *p += 801; return *q * 2; }
static int f2(int *p, int *q) { *p += 487; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[7]);
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    sum += f2(&cells[3], &cells[7]);
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    cells[7] = cells[2] + 575;
    cells[3] = cells[3] + 913;
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
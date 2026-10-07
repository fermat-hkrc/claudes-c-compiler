static int f1(int *p, int *q) { *p += 922; return *q * 2; }
static int f2(int *p, int *q) { *p += 434; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 773; cells[5] += arr[7 % 6];
    sum += f1(&cells[6], &cells[5]);
    cells[0] = cells[2] + 889;
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    sum += f2(&cells[6], &cells[4]);
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
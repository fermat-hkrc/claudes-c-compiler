static int f1(int *p, int *q) { *p += 354; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[6], &cells[5]);
    cells[2] = cells[1] + 406;
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    cells[5] = cells[6] + 730;
    arr[4 % 6] = 734; cells[4] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    cells[1] = cells[0] + 764;
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
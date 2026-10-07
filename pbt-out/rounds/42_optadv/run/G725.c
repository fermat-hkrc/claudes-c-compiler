static int f1(int *p, int *q) { *p += 608; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    sum += f1(&cells[4], &cells[4]);
    arr[5 % 6] = 684; cells[7] += arr[5 % 6];
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    cells[4] = cells[7] + 640;
    cells[7] = cells[2] + 706;
    arr[4 % 6] = 307; cells[7] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 632; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 374; cells[7] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    arr[5 % 6] = 262; cells[2] += arr[5 % 6];
    arr[0 % 6] = 438; cells[4] += arr[0 % 6];
    cells[2] = cells[2] + 491;
    arr[0 % 6] = 287; cells[0] += arr[0 % 6];
    sum += f1(&cells[0], &cells[7]);
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
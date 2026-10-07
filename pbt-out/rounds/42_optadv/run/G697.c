static int f1(int *p, int *q) { *p += 240; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    cells[6] = cells[4] + 706;
    arr[4 % 6] = 25; cells[1] += arr[4 % 6];
    sum += f1(&cells[4], &cells[0]);
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    arr[0 % 6] = 468; cells[7] += arr[0 % 6];
    cells[5] = cells[6] + 665;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
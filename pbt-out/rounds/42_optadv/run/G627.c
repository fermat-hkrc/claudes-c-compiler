static int f1(int *p, int *q) { *p += 104; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    arr[6 % 6] = 976; cells[3] += arr[6 % 6];
    sum += f1(&cells[2], &cells[0]);
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    arr[3 % 6] = 321; cells[5] += arr[3 % 6];
    cells[0] = cells[6] + 126;
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 981; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[7 % 6] = 696; cells[7] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[1] }; box.v += *box.np; cells[4] = box.v; }
    sum += f1(&cells[2], &cells[0]);
    cells[5] = cells[6] + 404;
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    cells[4] = cells[2] + 223;
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    cells[6] = cells[1] + 38;
    arr[1 % 6] = 673; cells[4] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 307; return *q * 2; }
static int f2(int *p, int *q) { *p += 663; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 542; cells[0] += arr[1 % 6];
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    sum += f1(&cells[0], &cells[4]);
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    cells[0] = cells[6] + 163;
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    arr[2 % 6] = 682; cells[1] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f2(&cells[2], &cells[2]);
    cells[6] = cells[7] + 555;
    arr[2 % 6] = 75; cells[0] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
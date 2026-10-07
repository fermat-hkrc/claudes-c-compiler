static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 467; cells[6] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    static int f1(int *p, int *q) { *p += 997; return *q * 2; }
    sum += f1(&cells[6], &cells[2]);
    arr[2 % 6] = 128; cells[2] += arr[2 % 6];
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    arr[3 % 6] = 201; cells[3] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    static int f2(int *p, int *q) { *p += 128; return *q * 2; }
    sum += f2(&cells[3], &cells[5]);
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    cells[0] = cells[2] + 477;
    cells[2] = cells[6] + 302;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
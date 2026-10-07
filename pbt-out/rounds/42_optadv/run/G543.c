static int f1(int *p, int *q) { *p += 576; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 142; cells[3] += arr[3 % 6];
    sum += f1(&cells[4], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[7] = cells[1] + 241;
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    arr[4 % 6] = 565; cells[3] += arr[4 % 6];
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    arr[6 % 6] = 835; cells[2] += arr[6 % 6];
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    arr[7 % 6] = 562; cells[3] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 68; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[3 % 6] = 384; cells[6] += arr[3 % 6];
    arr[6 % 6] = 507; cells[4] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    arr[1 % 6] = 454; cells[1] += arr[1 % 6];
    sum += f1(&cells[5], &cells[7]);
    arr[1 % 6] = 978; cells[7] += arr[1 % 6];
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
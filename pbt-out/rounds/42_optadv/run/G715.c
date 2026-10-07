static int f1(int *p, int *q) { *p += 972; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    arr[3 % 6] = 48; cells[1] += arr[3 % 6];
    cells[3] = cells[3] + 252;
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    arr[3 % 6] = 171; cells[5] += arr[3 % 6];
    cells[6] = cells[7] + 470;
    sum += f1(&cells[7], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[5 % 6] = 44; cells[7] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[6 % 6] = 818; cells[3] += arr[6 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
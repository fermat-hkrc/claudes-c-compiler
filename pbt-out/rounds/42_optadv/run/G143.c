static int f1(int *p, int *q) { *p += 235; return *q * 2; }
static int f2(int *p, int *q) { *p += 747; return *q * 2; }
static int f3(int *p, int *q) { *p += 878; return *q * 2; }
static int f4(int *p, int *q) { *p += 318; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 722; cells[6] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    sum += f1(&cells[2], &cells[4]);
    sum += f2(&cells[2], &cells[0]);
    arr[6 % 6] = 832; cells[5] += arr[6 % 6];
    arr[5 % 6] = 857; cells[0] += arr[5 % 6];
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    sum += f3(&cells[7], &cells[4]);
    arr[0 % 6] = 993; cells[7] += arr[0 % 6];
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    arr[2 % 6] = 501; cells[2] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    arr[1 % 6] = 927; cells[2] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    sum += f4(&cells[5], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
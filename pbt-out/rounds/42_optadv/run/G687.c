static int f1(int *p, int *q) { *p += 932; return *q * 2; }
static int f2(int *p, int *q) { *p += 198; return *q * 2; }
static int f3(int *p, int *q) { *p += 854; return *q * 2; }
static int f4(int *p, int *q) { *p += 597; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[3], &cells[5]);
    sum += cells[2] > cells[7] ? cells[2] : -cells[7];
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    sum += f2(&cells[3], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    sum += f3(&cells[0], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[3 % 6] = 885; cells[0] += arr[3 % 6];
    sum += f4(&cells[3], &cells[0]);
    cells[7] = cells[3] + 173;
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
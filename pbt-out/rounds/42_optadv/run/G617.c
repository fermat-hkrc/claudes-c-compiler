static int f1(int *p, int *q) { *p += 47; return *q * 2; }
static int f2(int *p, int *q) { *p += 96; return *q * 2; }
static int f3(int *p, int *q) { *p += 810; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[2], &cells[1] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    arr[3 % 6] = 942; cells[1] += arr[3 % 6];
    sum += f1(&cells[2], &cells[5]);
    arr[0 % 6] = 780; cells[0] += arr[0 % 6];
    sum += f2(&cells[4], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[5 % 6] = 717; cells[5] += arr[5 % 6];
    sum += f3(&cells[3], &cells[5]);
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[7 % 6] = 277; cells[1] += arr[7 % 6];
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    { struct { int v; int *np; } box = { cells[1], &cells[0] }; box.v += *box.np; cells[1] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
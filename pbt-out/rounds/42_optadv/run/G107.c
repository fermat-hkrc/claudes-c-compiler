static int f1(int *p, int *q) { *p += 978; return *q * 2; }
static int f2(int *p, int *q) { *p += 460; return *q * 2; }
static int f3(int *p, int *q) { *p += 761; return *q * 2; }
static int f4(int *p, int *q) { *p += 807; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[3] = cells[6] + 98;
    sum += cells[5] > cells[7] ? cells[5] : -cells[7];
    cells[7] = cells[6] + 960;
    arr[5 % 6] = 107; cells[0] += arr[5 % 6];
    cells[1] = cells[1] + 972;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f1(&cells[4], &cells[2]);
    arr[0 % 6] = 263; cells[0] += arr[0 % 6];
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    sum += f2(&cells[4], &cells[6]);
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    sum += f3(&cells[0], &cells[6]);
    arr[2 % 6] = 554; cells[1] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[5] }; box.v += *box.np; cells[7] = box.v; }
    sum += f4(&cells[2], &cells[4]);
    arr[5 % 6] = 278; cells[5] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[5 % 6] = 142; cells[7] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
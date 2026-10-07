static int f1(int *p, int *q) { *p += 865; return *q * 2; }
static int f2(int *p, int *q) { *p += 780; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[3] + 365;
    arr[6 % 6] = 381; cells[2] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    arr[7 % 6] = 657; cells[1] += arr[7 % 6];
    cells[4] = cells[2] + 126;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    cells[6] = cells[4] + 869;
    sum += f1(&cells[3], &cells[7]);
    sum += f2(&cells[1], &cells[0]);
    arr[5 % 6] = 0; cells[5] += arr[5 % 6];
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    cells[1] = cells[2] + 415;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
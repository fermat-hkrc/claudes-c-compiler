static int f1(int *p, int *q) { *p += 179; return *q * 2; }
static int f2(int *p, int *q) { *p += 342; return *q * 2; }
static int f3(int *p, int *q) { *p += 624; return *q * 2; }
static int f4(int *p, int *q) { *p += 29; return *q * 2; }
static int f5(int *p, int *q) { *p += 822; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[7], &cells[0]);
    sum += f2(&cells[0], &cells[4]);
    cells[4] = cells[4] + 50;
    sum += f3(&cells[7], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    arr[0 % 6] = 993; cells[3] += arr[0 % 6];
    arr[4 % 6] = 734; cells[3] += arr[4 % 6];
    arr[3 % 6] = 866; cells[5] += arr[3 % 6];
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    cells[4] = cells[3] + 95;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[4 % 6] = 73; cells[6] += arr[4 % 6];
    sum += f4(&cells[1], &cells[5]);
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    sum += f5(&cells[1], &cells[7]);
    cells[7] = cells[4] + 566;
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 929; return *q * 2; }
static int f2(int *p, int *q) { *p += 275; return *q * 2; }
static int f3(int *p, int *q) { *p += 917; return *q * 2; }
static int f4(int *p, int *q) { *p += 403; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[3], &cells[1]);
    arr[4 % 6] = 702; cells[5] += arr[4 % 6];
    arr[3 % 6] = 17; cells[0] += arr[3 % 6];
    sum += f2(&cells[7], &cells[7]);
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    sum += f3(&cells[2], &cells[1]);
    arr[5 % 6] = 879; cells[0] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    cells[0] = cells[3] + 926;
    cells[6] = cells[6] + 534;
    sum += f4(&cells[4], &cells[6]);
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    cells[3] = cells[5] + 273;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 545; return *q * 2; }
static int f2(int *p, int *q) { *p += 345; return *q * 2; }
static int f3(int *p, int *q) { *p += 41; return *q * 2; }
static int f4(int *p, int *q) { *p += 200; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    arr[0 % 6] = 758; cells[7] += arr[0 % 6];
    sum += f1(&cells[7], &cells[3]);
    sum += f2(&cells[5], &cells[3]);
    cells[0] = cells[6] + 609;
    sum += f3(&cells[2], &cells[7]);
    sum += f4(&cells[0], &cells[2]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
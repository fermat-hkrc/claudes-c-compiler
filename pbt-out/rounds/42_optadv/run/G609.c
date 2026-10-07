static int f1(int *p, int *q) { *p += 119; return *q * 2; }
static int f2(int *p, int *q) { *p += 605; return *q * 2; }
static int f3(int *p, int *q) { *p += 677; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 423u; }
    cells[4] = cells[7] + 920;
    sum += f1(&cells[2], &cells[7]);
    sum += f2(&cells[1], &cells[3]);
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[5 % 6] = 735; cells[1] += arr[5 % 6];
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    sum += f3(&cells[4], &cells[2]);
    arr[1 % 6] = 413; cells[4] += arr[1 % 6];
    arr[5 % 6] = 218; cells[7] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 366u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
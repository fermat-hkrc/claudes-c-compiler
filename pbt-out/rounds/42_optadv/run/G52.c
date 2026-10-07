static int f1(int *p, int *q) { *p += 105; return *q * 2; }
static int f2(int *p, int *q) { *p += 646; return *q * 2; }
static int f3(int *p, int *q) { *p += 656; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 523; cells[1] += arr[2 % 6];
    cells[3] = cells[6] + 122;
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    sum += f1(&cells[0], &cells[5]);
    sum += f2(&cells[2], &cells[2]);
    sum += f3(&cells[5], &cells[2]);
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    arr[5 % 6] = 20; cells[7] += arr[5 % 6];
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 491u; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    arr[2 % 6] = 208; cells[0] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
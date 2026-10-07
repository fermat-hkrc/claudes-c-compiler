static int f1(int *p, int *q) { *p += 827; return *q * 2; }
static int f2(int *p, int *q) { *p += 786; return *q * 2; }
static int f3(int *p, int *q) { *p += 651; return *q * 2; }
static int f4(int *p, int *q) { *p += 636; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[7], &cells[6]);
    arr[3 % 6] = 931; cells[0] += arr[3 % 6];
    arr[6 % 6] = 128; cells[2] += arr[6 % 6];
    sum += f2(&cells[0], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[6] = cells[3] + 798;
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 840u; }
    arr[5 % 6] = 432; cells[3] += arr[5 % 6];
    sum += f3(&cells[0], &cells[1]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    sum += f4(&cells[7], &cells[0]);
    cells[5] = cells[7] + 788;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
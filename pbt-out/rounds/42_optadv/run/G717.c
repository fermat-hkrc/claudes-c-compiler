static int f1(int *p, int *q) { *p += 571; return *q * 2; }
static int f2(int *p, int *q) { *p += 452; return *q * 2; }
static int f3(int *p, int *q) { *p += 738; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    arr[0 % 6] = 159; cells[4] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    sum += f1(&cells[0], &cells[2]);
    arr[4 % 6] = 183; cells[1] += arr[4 % 6];
    arr[1 % 6] = 627; cells[3] += arr[1 % 6];
    sum += f2(&cells[4], &cells[6]);
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 837u; }
    arr[0 % 6] = 126; cells[2] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    sum += f3(&cells[3], &cells[3]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
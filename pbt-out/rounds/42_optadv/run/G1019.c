static int f1(int *p, int *q) { *p += 946; return *q * 2; }
static int f2(int *p, int *q) { *p += 30; return *q * 2; }
static int f3(int *p, int *q) { *p += 247; return *q * 2; }
static int f4(int *p, int *q) { *p += 710; return *q * 2; }
static int f5(int *p, int *q) { *p += 683; return *q * 2; }
static int f6(int *p, int *q) { *p += 866; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    arr[4 % 6] = 585; cells[5] += arr[4 % 6];
    sum += f1(&cells[2], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 271u; }
    arr[0 % 6] = 283; cells[0] += arr[0 % 6];
    sum += f2(&cells[2], &cells[1]);
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 566u; }
    arr[4 % 6] = 54; cells[1] += arr[4 % 6];
    sum += f3(&cells[3], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += f4(&cells[0], &cells[0]);
    arr[5 % 6] = 427; cells[3] += arr[5 % 6];
    arr[1 % 6] = 696; cells[2] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    arr[4 % 6] = 643; cells[4] += arr[4 % 6];
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 498u; }
    arr[2 % 6] = 555; cells[3] += arr[2 % 6];
    sum += f5(&cells[6], &cells[5]);
    arr[6 % 6] = 890; cells[5] += arr[6 % 6];
    cells[5] = cells[2] + 595;
    sum += f6(&cells[7], &cells[7]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
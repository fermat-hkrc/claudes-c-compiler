static int f1(int *p, int *q) { *p += 160; return *q * 2; }
static int f2(int *p, int *q) { *p += 33; return *q * 2; }
static int f3(int *p, int *q) { *p += 583; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[4] = cells[0] + 697;
    arr[4 % 6] = 756; cells[6] += arr[4 % 6];
    sum += f1(&cells[1], &cells[7]);
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    sum += f2(&cells[1], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f3(&cells[0], &cells[1]);
    arr[5 % 6] = 599; cells[4] += arr[5 % 6];
    arr[7 % 6] = 428; cells[7] += arr[7 % 6];
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 295u; }
    cells[5] = cells[0] + 340;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 563u; }
    cells[2] = cells[1] + 393;
    arr[5 % 6] = 392; cells[6] += arr[5 % 6];
    arr[7 % 6] = 695; cells[2] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
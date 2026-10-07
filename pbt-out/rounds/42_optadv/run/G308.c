static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 761u; }
    arr[7 % 6] = 285; cells[6] += arr[7 % 6];
    cells[2] = cells[5] + 524;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[4 % 6] = 778; cells[7] += arr[4 % 6];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 217u; }
    arr[6 % 6] = 86; cells[1] += arr[6 % 6];
    cells[3] = cells[1] + 131;
    arr[3 % 6] = 440; cells[2] += arr[3 % 6];
    arr[3 % 6] = 127; cells[5] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 114; return *q * 2; }
    sum += f1(&cells[2], &cells[7]);
    static int f2(int *p, int *q) { *p += 862; return *q * 2; }
    sum += f2(&cells[0], &cells[2]);
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    static int f3(int *p, int *q) { *p += 531; return *q * 2; }
    sum += f3(&cells[2], &cells[1]);
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 338u; }
    arr[5 % 6] = 450; cells[1] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
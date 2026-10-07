static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    static int f1(int *p, int *q) { *p += 355; return *q * 2; }
    sum += f1(&cells[1], &cells[6]);
    static int f2(int *p, int *q) { *p += 357; return *q * 2; }
    sum += f2(&cells[1], &cells[0]);
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 611u; }
    { struct { int v; int *np; } box = { cells[5], &cells[5] }; box.v += *box.np; cells[5] = box.v; }
    arr[3 % 6] = 620; cells[7] += arr[3 % 6];
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    arr[4 % 6] = 400; cells[0] += arr[4 % 6];
    static int f3(int *p, int *q) { *p += 562; return *q * 2; }
    sum += f3(&cells[5], &cells[0]);
    static int f4(int *p, int *q) { *p += 525; return *q * 2; }
    sum += f4(&cells[3], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[6] = cells[5] + 455;
    { int *p = &cells[3]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 960u; }
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 298u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
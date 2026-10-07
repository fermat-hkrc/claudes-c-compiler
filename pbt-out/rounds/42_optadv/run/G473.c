static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[2 % 6] = 918; cells[6] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    cells[2] = cells[6] + 836;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[4] = cells[7] + 584;
    static int f1(int *p, int *q) { *p += 207; return *q * 2; }
    sum += f1(&cells[2], &cells[4]);
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    cells[7] = cells[5] + 178;
    sum += cells[7] > cells[1] ? cells[7] : -cells[1];
    cells[1] = cells[6] + 641;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    arr[2 % 6] = 919; cells[4] += arr[2 % 6];
    arr[7 % 6] = 473; cells[3] += arr[7 % 6];
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 668u; }
    static int f2(int *p, int *q) { *p += 708; return *q * 2; }
    sum += f2(&cells[4], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    static int f3(int *p, int *q) { *p += 392; return *q * 2; }
    sum += f3(&cells[6], &cells[1]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 333; return *q * 2; }
static int f2(int *p, int *q) { *p += 792; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[2] + 421;
    arr[1 % 6] = 686; cells[6] += arr[1 % 6];
    cells[3] = cells[4] + 556;
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 414u; }
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 138u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f1(&cells[0], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[0 % 6] = 335; cells[6] += arr[0 % 6];
    sum += f2(&cells[0], &cells[7]);
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    cells[0] = cells[2] + 30;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[2] = cells[4] + 114;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[2] = cells[4] + 651;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[6 % 6] = 494; cells[4] += arr[6 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 408; return *q * 2; }
static int f2(int *p, int *q) { *p += 51; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 739; cells[5] += arr[5 % 6];
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 526u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[1 % 6] = 619; cells[4] += arr[1 % 6];
    cells[6] = cells[2] + 111;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 79u; }
    arr[6 % 6] = 608; cells[1] += arr[6 % 6];
    sum += f1(&cells[2], &cells[4]);
    arr[1 % 6] = 755; cells[0] += arr[1 % 6];
    sum += f2(&cells[5], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    cells[0] = cells[1] + 822;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
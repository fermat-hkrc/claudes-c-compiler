static int f1(int *p, int *q) { *p += 477; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[6 % 6] = 872; cells[5] += arr[6 % 6];
    cells[1] = cells[6] + 728;
    cells[3] = cells[5] + 987;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    sum += f1(&cells[6], &cells[3]);
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 378u; }
    { struct { int v; int *np; } box = { cells[0], &cells[7] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
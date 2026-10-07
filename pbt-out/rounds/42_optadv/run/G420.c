static int f1(int *p, int *q) { *p += 235; return *q * 2; }
static int f2(int *p, int *q) { *p += 419; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[6] + 56;
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 195u; }
    sum += f1(&cells[4], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[4 % 6] = 831; cells[2] += arr[4 % 6];
    arr[6 % 6] = 494; cells[1] += arr[6 % 6];
    sum += f2(&cells[3], &cells[4]);
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    arr[1 % 6] = 844; cells[4] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[6 % 6] = 811; cells[3] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    cells[7] = cells[6] + 943;
    sum += cells[2] > cells[6] ? cells[2] : -cells[6];
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[1], &cells[6] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 673u; }
    arr[3 % 6] = 242; cells[1] += arr[3 % 6];
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 132; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f1(&cells[0], &cells[0]);
    arr[0 % 6] = 433; cells[0] += arr[0 % 6];
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[2] = cells[1] + 333;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[7 % 6] = 271; cells[0] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    arr[5 % 6] = 501; cells[7] += arr[5 % 6];
    arr[6 % 6] = 977; cells[0] += arr[6 % 6];
    arr[4 % 6] = 476; cells[3] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 351u; }
    cells[5] = cells[7] + 62;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
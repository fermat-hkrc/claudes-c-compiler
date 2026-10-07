static int f1(int *p, int *q) { *p += 650; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += cells[2] > cells[3] ? cells[2] : -cells[3];
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 783u; }
    arr[7 % 6] = 946; cells[7] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[4] > cells[0] ? cells[4] : -cells[0];
    arr[1 % 6] = 265; cells[6] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    sum += f1(&cells[1], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[1] = cells[2] + 906;
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 644u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
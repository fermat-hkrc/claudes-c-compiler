static int f1(int *p, int *q) { *p += 203; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    sum += f1(&cells[3], &cells[3]);
    cells[3] = cells[0] + 6;
    arr[5 % 6] = 529; cells[2] += arr[5 % 6];
    sum += cells[4] > cells[6] ? cells[4] : -cells[6];
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 506u; }
    cells[5] = cells[6] + 137;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
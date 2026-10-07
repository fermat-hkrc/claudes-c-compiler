static int f1(int *p, int *q) { *p += 746; return *q * 2; }
static int f2(int *p, int *q) { *p += 429; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[1], &cells[5]);
    arr[1 % 6] = 946; cells[1] += arr[1 % 6];
    sum += f2(&cells[7], &cells[5]);
    arr[3 % 6] = 443; cells[1] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 765u; }
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    arr[2 % 6] = 146; cells[5] += arr[2 % 6];
    arr[7 % 6] = 969; cells[7] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[3] > cells[4] ? cells[3] : -cells[4];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[1] > cells[7] ? cells[1] : -cells[7];
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    cells[3] = cells[6] + 127;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
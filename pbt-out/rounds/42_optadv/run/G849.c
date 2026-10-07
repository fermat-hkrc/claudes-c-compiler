static int f1(int *p, int *q) { *p += 753; return *q * 2; }
static int f2(int *p, int *q) { *p += 790; return *q * 2; }
static int f3(int *p, int *q) { *p += 491; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[7], &cells[6]);
    sum += f2(&cells[3], &cells[3]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[3 % 6] = 479; cells[4] += arr[3 % 6];
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 737u; }
    sum += f3(&cells[1], &cells[0]);
    { struct { int v; int *np; } box = { cells[5], &cells[1] }; box.v += *box.np; cells[5] = box.v; }
    arr[6 % 6] = 383; cells[6] += arr[6 % 6];
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    arr[6 % 6] = 79; cells[5] += arr[6 % 6];
    sum += cells[7] > cells[7] ? cells[7] : -cells[7];
    arr[3 % 6] = 940; cells[0] += arr[3 % 6];
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 734u; }
    { int *p = &cells[7]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 202u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[2 % 6] = 893; cells[6] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    arr[6 % 6] = 999; cells[6] += arr[6 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
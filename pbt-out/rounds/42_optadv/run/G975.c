static int f1(int *p, int *q) { *p += 260; return *q * 2; }
static int f2(int *p, int *q) { *p += 312; return *q * 2; }
static int f3(int *p, int *q) { *p += 349; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    { int *p = &cells[7]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 941u; }
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 885u; }
    sum += f1(&cells[0], &cells[1]);
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    sum += f2(&cells[1], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    arr[1 % 6] = 69; cells[5] += arr[1 % 6];
    arr[1 % 6] = 655; cells[6] += arr[1 % 6];
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 104u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[1 % 6] = 913; cells[5] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[3 % 6] = 395; cells[7] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    arr[1 % 6] = 126; cells[5] += arr[1 % 6];
    sum += f3(&cells[4], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
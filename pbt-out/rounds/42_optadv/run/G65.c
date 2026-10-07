static int f1(int *p, int *q) { *p += 568; return *q * 2; }
static int f2(int *p, int *q) { *p += 951; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 561u; }
    sum += f1(&cells[7], &cells[6]);
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 724u; }
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    sum += cells[1] > cells[1] ? cells[1] : -cells[1];
    sum += f2(&cells[7], &cells[3]);
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    arr[1 % 6] = 939; cells[7] += arr[1 % 6];
    arr[0 % 6] = 87; cells[4] += arr[0 % 6];
    cells[2] = cells[6] + 696;
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    arr[0 % 6] = 90; cells[7] += arr[0 % 6];
    arr[5 % 6] = 254; cells[6] += arr[5 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    arr[7 % 6] = 990; cells[3] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[5 % 6] = 655; cells[3] += arr[5 % 6];
    arr[2 % 6] = 414; cells[5] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
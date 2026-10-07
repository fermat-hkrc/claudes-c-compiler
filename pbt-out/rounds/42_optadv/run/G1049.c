static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 281; cells[2] += arr[7 % 6];
    arr[2 % 6] = 951; cells[4] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    static int f1(int *p, int *q) { *p += 632; return *q * 2; }
    sum += f1(&cells[6], &cells[2]);
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    cells[1] = cells[6] + 777;
    arr[7 % 6] = 767; cells[1] += arr[7 % 6];
    arr[2 % 6] = 63; cells[2] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    cells[6] = cells[1] + 882;
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 310u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
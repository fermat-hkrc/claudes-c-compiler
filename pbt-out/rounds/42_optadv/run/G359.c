static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[3 % 6] = 144; cells[3] += arr[3 % 6];
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    arr[5 % 6] = 578; cells[5] += arr[5 % 6];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    static int f1(int *p, int *q) { *p += 575; return *q * 2; }
    sum += f1(&cells[7], &cells[2]);
    cells[1] = cells[6] + 619;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    arr[6 % 6] = 908; cells[6] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    static int f2(int *p, int *q) { *p += 250; return *q * 2; }
    sum += f2(&cells[5], &cells[3]);
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    cells[1] = cells[0] + 70;
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 197u; }
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    cells[5] = cells[4] + 329;
    cells[6] = cells[3] + 474;
    static int f3(int *p, int *q) { *p += 221; return *q * 2; }
    sum += f3(&cells[6], &cells[4]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
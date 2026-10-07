static int f1(int *p, int *q) { *p += 982; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    arr[6 % 6] = 990; cells[1] += arr[6 % 6];
    arr[2 % 6] = 745; cells[6] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    arr[4 % 6] = 279; cells[5] += arr[4 % 6];
    cells[3] = cells[0] + 572;
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    cells[7] = cells[6] + 931;
    arr[2 % 6] = 899; cells[5] += arr[2 % 6];
    cells[1] = cells[4] + 282;
    { struct { int v; int *np; } box = { cells[4], &cells[5] }; box.v += *box.np; cells[4] = box.v; }
    arr[0 % 6] = 868; cells[5] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 585u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    sum += f1(&cells[1], &cells[0]);
    arr[4 % 6] = 442; cells[0] += arr[4 % 6];
    { int *p = &cells[1]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 211u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
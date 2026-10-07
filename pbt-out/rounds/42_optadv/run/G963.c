static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 799u; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    static int f1(int *p, int *q) { *p += 978; return *q * 2; }
    sum += f1(&cells[2], &cells[2]);
    cells[4] = cells[5] + 296;
    arr[5 % 6] = 278; cells[2] += arr[5 % 6];
    arr[0 % 6] = 922; cells[7] += arr[0 % 6];
    arr[6 % 6] = 254; cells[3] += arr[6 % 6];
    arr[5 % 6] = 400; cells[4] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 581u; }
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    arr[4 % 6] = 514; cells[5] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[1 % 6] = 801; cells[2] += arr[1 % 6];
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    arr[7 % 6] = 713; cells[7] += arr[7 % 6];
    arr[1 % 6] = 752; cells[4] += arr[1 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
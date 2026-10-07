static int f1(int *p, int *q) { *p += 584; return *q * 2; }
static int f2(int *p, int *q) { *p += 451; return *q * 2; }
static int f3(int *p, int *q) { *p += 619; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[7], &cells[4]);
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 438u; }
    cells[2] = cells[0] + 957;
    sum += f2(&cells[1], &cells[3]);
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    sum += cells[7] > cells[0] ? cells[7] : -cells[0];
    arr[2 % 6] = 894; cells[6] += arr[2 % 6];
    { int *p = &cells[2]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 302u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    arr[1 % 6] = 571; cells[1] += arr[1 % 6];
    cells[0] = cells[3] + 855;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[4 % 6] = 834; cells[5] += arr[4 % 6];
    sum += f3(&cells[6], &cells[1]);
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
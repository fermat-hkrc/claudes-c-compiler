static int f1(int *p, int *q) { *p += 262; return *q * 2; }
static int f2(int *p, int *q) { *p += 48; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 629; cells[5] += arr[0 % 6];
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 217u; }
    arr[4 % 6] = 155; cells[0] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[4] = cells[7] + 463;
    arr[0 % 6] = 922; cells[4] += arr[0 % 6];
    cells[6] = cells[2] + 153;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[0] = cells[2] + 147;
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    sum += f1(&cells[0], &cells[0]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    arr[7 % 6] = 990; cells[4] += arr[7 % 6];
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    arr[5 % 6] = 777; cells[2] += arr[5 % 6];
    arr[0 % 6] = 282; cells[2] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f2(&cells[6], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
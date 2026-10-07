static int f1(int *p, int *q) { *p += 625; return *q * 2; }
static int f2(int *p, int *q) { *p += 790; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 849; cells[2] += arr[5 % 6];
    sum += f1(&cells[0], &cells[1]);
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    arr[2 % 6] = 393; cells[3] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[0] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 441u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 217u; }
    cells[5] = cells[3] + 80;
    cells[2] = cells[5] + 423;
    sum += f2(&cells[4], &cells[2]);
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
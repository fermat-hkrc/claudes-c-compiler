static int f1(int *p, int *q) { *p += 217; return *q * 2; }
static int f2(int *p, int *q) { *p += 579; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[5], &cells[1]);
    { int *p = &cells[1]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 172u; }
    sum += f2(&cells[5], &cells[5]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    arr[7 % 6] = 150; cells[5] += arr[7 % 6];
    cells[0] = cells[0] + 856;
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
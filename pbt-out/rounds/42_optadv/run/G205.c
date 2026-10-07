static int f1(int *p, int *q) { *p += 298; return *q * 2; }
static int f2(int *p, int *q) { *p += 400; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[5 % 6] = 951; cells[2] += arr[5 % 6];
    cells[3] = cells[3] + 893;
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    sum += f1(&cells[5], &cells[4]);
    sum += f2(&cells[3], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[1 % 6] = 74; cells[4] += arr[1 % 6];
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 626u; }
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    sum += cells[4] > cells[1] ? cells[4] : -cells[1];
    sum += cells[3] > cells[0] ? cells[3] : -cells[0];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
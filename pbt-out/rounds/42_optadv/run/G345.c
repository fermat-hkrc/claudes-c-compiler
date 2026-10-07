static int f1(int *p, int *q) { *p += 708; return *q * 2; }
static int f2(int *p, int *q) { *p += 780; return *q * 2; }
static int f3(int *p, int *q) { *p += 244; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[1] ? cells[6] : -cells[1];
    cells[7] = cells[3] + 665;
    sum += f1(&cells[4], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[0 % 6] = 907; cells[7] += arr[0 % 6];
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    { int *p = &cells[5]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 935u; }
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    arr[7 % 6] = 466; cells[7] += arr[7 % 6];
    sum += f2(&cells[0], &cells[0]);
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += f3(&cells[6], &cells[6]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
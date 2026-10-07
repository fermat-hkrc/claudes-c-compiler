static int f1(int *p, int *q) { *p += 718; return *q * 2; }
static int f2(int *p, int *q) { *p += 871; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[1 % 6] = 939; cells[6] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[0] }; box.v += *box.np; cells[6] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f1(&cells[3], &cells[1]);
    sum += cells[7] > cells[5] ? cells[7] : -cells[5];
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    sum += f2(&cells[3], &cells[7]);
    arr[5 % 6] = 204; cells[7] += arr[5 % 6];
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    arr[7 % 6] = 617; cells[3] += arr[7 % 6];
    arr[3 % 6] = 594; cells[7] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[0]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 493u; }
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 583; return *q * 2; }
static int f2(int *p, int *q) { *p += 684; return *q * 2; }
static int f3(int *p, int *q) { *p += 923; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    sum += f1(&cells[5], &cells[0]);
    arr[3 % 6] = 568; cells[3] += arr[3 % 6];
    sum += f2(&cells[7], &cells[2]);
    arr[7 % 6] = 696; cells[7] += arr[7 % 6];
    sum += cells[7] > cells[4] ? cells[7] : -cells[4];
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    { struct { int v; int *np; } box = { cells[3], &cells[3] }; box.v += *box.np; cells[3] = box.v; }
    sum += f3(&cells[3], &cells[6]);
    { struct { int v; int *np; } box = { cells[7], &cells[7] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 78u; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[4 % 6] = 844; cells[0] += arr[4 % 6];
    arr[2 % 6] = 838; cells[0] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
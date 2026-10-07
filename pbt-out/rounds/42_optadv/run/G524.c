static int f1(int *p, int *q) { *p += 869; return *q * 2; }
static int f2(int *p, int *q) { *p += 883; return *q * 2; }
static int f3(int *p, int *q) { *p += 548; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[3], &cells[7] }; box.v += *box.np; cells[3] = box.v; }
    cells[1] = cells[2] + 636;
    arr[2 % 6] = 126; cells[4] += arr[2 % 6];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    arr[1 % 6] = 358; cells[0] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[1] }; box.v += *box.np; cells[0] = box.v; }
    arr[0 % 6] = 3; cells[6] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    sum += f1(&cells[3], &cells[3]);
    cells[4] = cells[7] + 698;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[2] > cells[4] ? cells[2] : -cells[4];
    cells[7] = cells[3] + 633;
    sum += f2(&cells[1], &cells[4]);
    arr[2 % 6] = 596; cells[4] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { int *p = &cells[6]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 120u; }
    sum += cells[0] > cells[7] ? cells[0] : -cells[7];
    sum += f3(&cells[6], &cells[6]);
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
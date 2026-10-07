static int f1(int *p, int *q) { *p += 140; return *q * 2; }
static int f2(int *p, int *q) { *p += 966; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    cells[1] = cells[1] + 883;
    arr[1 % 6] = 997; cells[4] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 638u; }
    sum += f1(&cells[1], &cells[0]);
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    arr[6 % 6] = 523; cells[2] += arr[6 % 6];
    sum += f2(&cells[4], &cells[1]);
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 969u; }
    { struct { int v; int *np; } box = { cells[4], &cells[6] }; box.v += *box.np; cells[4] = box.v; }
    arr[1 % 6] = 885; cells[1] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[2] }; box.v += *box.np; cells[0] = box.v; }
    cells[6] = cells[2] + 645;
    cells[7] = cells[1] + 369;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
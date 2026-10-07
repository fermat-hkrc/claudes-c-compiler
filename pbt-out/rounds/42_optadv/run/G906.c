static int f1(int *p, int *q) { *p += 781; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[4] = cells[0] + 221;
    cells[0] = cells[0] + 367;
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    arr[4 % 6] = 857; cells[7] += arr[4 % 6];
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += f1(&cells[1], &cells[4]);
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    { int *p = &cells[2]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 29u; }
    { struct { int v; int *np; } box = { cells[4], &cells[2] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    arr[0 % 6] = 925; cells[3] += arr[0 % 6];
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 169u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
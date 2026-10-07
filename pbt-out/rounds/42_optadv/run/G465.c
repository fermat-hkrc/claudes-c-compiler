static int f1(int *p, int *q) { *p += 838; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[6] > cells[2] ? cells[6] : -cells[2];
    sum += f1(&cells[6], &cells[7]);
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 441u; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    arr[2 % 6] = 152; cells[1] += arr[2 % 6];
    arr[5 % 6] = 638; cells[3] += arr[5 % 6];
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 384u; }
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    sum += cells[5] > cells[2] ? cells[5] : -cells[2];
    { struct { int v; int *np; } box = { cells[3], &cells[6] }; box.v += *box.np; cells[3] = box.v; }
    cells[0] = cells[5] + 678;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
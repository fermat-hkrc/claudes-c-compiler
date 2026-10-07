static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    arr[6 % 6] = 635; cells[7] += arr[6 % 6];
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    arr[0 % 6] = 640; cells[4] += arr[0 % 6];
    static int f1(int *p, int *q) { *p += 600; return *q * 2; }
    sum += f1(&cells[7], &cells[2]);
    arr[3 % 6] = 536; cells[3] += arr[3 % 6];
    { int *p = &cells[4]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 778u; }
    { struct { int v; int *np; } box = { cells[4], &cells[0] }; box.v += *box.np; cells[4] = box.v; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    arr[2 % 6] = 592; cells[6] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[5] }; box.v += *box.np; cells[0] = box.v; }
    cells[0] = cells[6] + 985;
    cells[6] = cells[6] + 915;
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 400u; }
    sum += cells[5] > cells[6] ? cells[5] : -cells[6];
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 818u; }
    arr[3 % 6] = 251; cells[6] += arr[3 % 6];
    sum += cells[6] > cells[4] ? cells[6] : -cells[4];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
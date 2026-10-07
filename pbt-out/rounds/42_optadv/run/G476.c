static int f1(int *p, int *q) { *p += 724; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    sum += cells[4] > cells[2] ? cells[4] : -cells[2];
    arr[4 % 6] = 962; cells[5] += arr[4 % 6];
    arr[6 % 6] = 447; cells[2] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 447u; }
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 265u; }
    cells[0] = cells[4] + 769;
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 423u; }
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    arr[5 % 6] = 346; cells[7] += arr[5 % 6];
    sum += f1(&cells[7], &cells[2]);
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[2]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 997u; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
static int f1(int *p, int *q) { *p += 647; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    cells[0] = cells[4] + 90;
    { int *p = &cells[3]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 676u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { struct { int v; int *np; } box = { cells[3], &cells[1] }; box.v += *box.np; cells[3] = box.v; }
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 282u; }
    cells[2] = cells[4] + 151;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[3 % 6] = 133; cells[6] += arr[3 % 6];
    cells[7] = cells[6] + 675;
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    cells[1] = cells[2] + 156;
    sum += cells[2] > cells[1] ? cells[2] : -cells[1];
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    sum += f1(&cells[7], &cells[3]);
    cells[3] = cells[2] + 449;
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[6] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 764u; }
    arr[3 % 6] = 809; cells[4] += arr[3 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
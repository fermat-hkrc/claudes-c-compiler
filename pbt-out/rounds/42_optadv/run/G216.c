static int f1(int *p, int *q) { *p += 692; return *q * 2; }
static int f2(int *p, int *q) { *p += 951; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 48u; }
    { int *p = &cells[1]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 647u; }
    { struct { int v; int *np; } box = { cells[5], &cells[4] }; box.v += *box.np; cells[5] = box.v; }
    sum += f1(&cells[6], &cells[0]);
    arr[2 % 6] = 637; cells[5] += arr[2 % 6];
    { int *p = &cells[5]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 202u; }
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    arr[4 % 6] = 686; cells[0] += arr[4 % 6];
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 823u; }
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 900u; }
    sum += cells[0] > cells[2] ? cells[0] : -cells[2];
    sum += cells[1] > cells[4] ? cells[1] : -cells[4];
    cells[0] = cells[5] + 99;
    sum += cells[0] > cells[1] ? cells[0] : -cells[1];
    sum += f2(&cells[1], &cells[5]);
    { int *p = &cells[2]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 172u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
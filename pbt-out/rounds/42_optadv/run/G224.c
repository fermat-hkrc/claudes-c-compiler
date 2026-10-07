static int f1(int *p, int *q) { *p += 957; return *q * 2; }
static int f2(int *p, int *q) { *p += 70; return *q * 2; }
static int f3(int *p, int *q) { *p += 176; return *q * 2; }
static int f4(int *p, int *q) { *p += 96; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[4], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[6 % 6] = 576; cells[2] += arr[6 % 6];
    arr[7 % 6] = 125; cells[3] += arr[7 % 6];
    { int *p = &cells[1]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 323u; }
    sum += f2(&cells[1], &cells[2]);
    cells[6] = cells[6] + 403;
    cells[6] = cells[6] + 489;
    cells[3] = cells[3] + 479;
    { struct { int v; int *np; } box = { cells[0], &cells[0] }; box.v += *box.np; cells[0] = box.v; }
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    cells[0] = cells[1] + 422;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f3(&cells[4], &cells[3]);
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    sum += f4(&cells[1], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
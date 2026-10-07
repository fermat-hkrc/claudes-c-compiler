static int f1(int *p, int *q) { *p += 490; return *q * 2; }
static int f2(int *p, int *q) { *p += 144; return *q * 2; }
static int f3(int *p, int *q) { *p += 101; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[7] = cells[5] + 494;
    arr[4 % 6] = 983; cells[4] += arr[4 % 6];
    sum += f1(&cells[5], &cells[7]);
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[6] > cells[0] ? cells[6] : -cells[0];
    cells[5] = cells[7] + 301;
    cells[2] = cells[6] + 913;
    { int *p = &cells[3]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 678u; }
    sum += f2(&cells[6], &cells[7]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += f3(&cells[3], &cells[1]);
    sum += cells[1] > cells[2] ? cells[1] : -cells[2];
    cells[3] = cells[4] + 122;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
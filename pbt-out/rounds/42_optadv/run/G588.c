static int f1(int *p, int *q) { *p += 435; return *q * 2; }
static int f2(int *p, int *q) { *p += 823; return *q * 2; }
static int f3(int *p, int *q) { *p += 674; return *q * 2; }
static int f4(int *p, int *q) { *p += 405; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[4], &cells[3]);
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    arr[7 % 6] = 379; cells[6] += arr[7 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    cells[6] = cells[4] + 202;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 646u; }
    sum += f2(&cells[6], &cells[1]);
    sum += f3(&cells[0], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    arr[7 % 6] = 609; cells[1] += arr[7 % 6];
    sum += f4(&cells[5], &cells[0]);
    { struct { int v; int *np; } box = { cells[3], &cells[4] }; box.v += *box.np; cells[3] = box.v; }
    cells[1] = cells[0] + 265;
    cells[3] = cells[5] + 486;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
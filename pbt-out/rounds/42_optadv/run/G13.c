static int f1(int *p, int *q) { *p += 667; return *q * 2; }
static int f2(int *p, int *q) { *p += 782; return *q * 2; }
static int f3(int *p, int *q) { *p += 18; return *q * 2; }
static int f4(int *p, int *q) { *p += 351; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 682u; }
    arr[4 % 6] = 381; cells[7] += arr[4 % 6];
    cells[2] = cells[5] + 629;
    sum += f1(&cells[6], &cells[0]);
    { int *p = &cells[4]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 193u; }
    { int *p = &cells[7]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 834u; }
    arr[4 % 6] = 620; cells[4] += arr[4 % 6];
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    sum += f2(&cells[4], &cells[6]);
    sum += f3(&cells[3], &cells[7]);
    { struct { int v; int *np; } box = { cells[6], &cells[3] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[4]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 173u; }
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    sum += f4(&cells[5], &cells[2]);
    sum += cells[5] > cells[4] ? cells[5] : -cells[4];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 7) % 8] % 97;
    arr[6 % 6] = 883; cells[1] += arr[6 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[4] = cells[3] + 498;
    arr[0 % 6] = 923; cells[1] += arr[0 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }
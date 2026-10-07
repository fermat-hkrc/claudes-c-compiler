static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    static int f1(int *p, int *q) { *p += 594; return *q * 2; }
    sum += f1(&cells[3], &cells[4]);
    static int f2(int *p, int *q) { *p += 575; return *q * 2; }
    sum += f2(&cells[2], &cells[2]);
    cells[0] = cells[5] + 355;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    cells[3] = cells[6] + 698;
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 799u; }
    cells[5] = cells[4] + 585;
    arr[0 % 6] = 124; cells[4] += arr[0 % 6];
    { int *p = &cells[6]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 806u; }
    static int f3(int *p, int *q) { *p += 740; return *q * 2; }
    sum += f3(&cells[1], &cells[4]);
    { int *p = &cells[0]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 845u; }
    sum += cells[2] > cells[5] ? cells[2] : -cells[5];
    sum += cells[1] > cells[0] ? cells[1] : -cells[0];
    static int f4(int *p, int *q) { *p += 132; return *q * 2; }
    sum += f4(&cells[4], &cells[2]);
    arr[2 % 6] = 87; cells[4] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[2] = cells[6] + 815;
    { struct { int v; int *np; } box = { cells[2], &cells[2] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }